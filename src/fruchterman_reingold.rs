///
/// Implements the force-directed graph layout algorithm as
/// proposed by Fruchterman and Reingold [1].
///
/// [1]: http://emr.cs.iit.edu/~reingold/force-directed.pdf
///

use super::{P2d, Vector};

// k_s == l
#[inline]
fn attractive_force<V>(p1: &V, p2: &V, k_s: f32) -> V
    where V: Vector<Scalar = f32>
{
    let force = p1.sub(p2);
    let length = force.length_squared().sqrt();
    let strength = length / k_s;
    return force.scale(strength);
}

// k_r == l^2
#[inline]
fn repulsive_force<V>(p1: &V, p2: &V, k_r: f32) -> V
    where V: Vector<Scalar = f32>
{
    let force = p1.sub(p2);
    let length_squared = force.length_squared();
    if length_squared > 0.0 {
        let strength = k_r / length_squared;
        return force.scale(strength);
    }
    return force;
}

pub trait ForceDirected<V> where V: Vector<Scalar = f32>
{
    fn reset_forces(&mut self);

    // adds result of `f` to force1 and substracts from force2
    fn update_force_each_node_pair<F: Fn(&V, &V) -> V>(&mut self, f: F);

    // adds result of `f` to force1 and substracts from force2
    fn update_force_each_edge<F: Fn(&V, &V) -> V>(&mut self, f: F);

    fn update_positions<F: FnMut(&V, &V) -> V>(&mut self, f: F);
}

fn iterate<V, FD>(fd: &mut FD, step: f32, k_r: f32, k_s: f32, min_pos: &V, max_pos: &V) -> f32
    where V: Vector<Scalar = f32>,
          FD: ForceDirected<V>
{

    // Reset all forces to zero.
    fd.reset_forces();

    // Calculate repulsive force between all pairs.
    fd.update_force_each_node_pair(|pos1, pos2| repulsive_force(pos1, pos2, k_r));

    // Calculate spring force between adjacent pairs (edges).
    fd.update_force_each_edge(|pos1, pos2| attractive_force(pos1, pos2, k_s).scale(-1.0));

    // update positions
    let mut sum_distance = 0.0;

    fd.update_positions(|position, force| {
        let mut new_pos = position.clone();

        let length = force.length_squared().sqrt();
        if length > 0.0 {
            new_pos.add_scaled(step / length, &force);

            // add up the moved distance. we move by step.
            sum_distance += step;
        }
        new_pos.clip_within(min_pos, max_pos)
    });

    return sum_distance;
}

pub fn layout<V, FD, F>(fd: &mut FD,
                        step_fn: F,
                        max_iter: usize,
                        converge_eps: f32,
                        k_r: f32,
                        k_s: f32,
                        min_pos: &V,
                        max_pos: &V)
    where V: Vector<Scalar = f32>,
          FD: ForceDirected<V>,
          F: Fn(usize) -> f32
{
    let mut iter: usize = 0;
    while iter < max_iter {
        let step = step_fn(iter);
        iter += 1;

        let dist_moved = iterate(fd, step, k_r, k_s, min_pos, max_pos);
        if dist_moved < converge_eps {
            break;
        }
    }
}

struct Layout<'a, 'b, V: 'a> {
    forces: Vec<V>,
    node_positions: &'a mut Vec<V>,
    node_neighbors: &'b [Vec<usize>],
    lock_first_n_positions: usize,
}

impl<'a, 'b, V> Layout<'a, 'b, V>
    where V: Vector<Scalar = f32>
{
    fn new<'c, 'd>(node_positions: &'c mut Vec<V>,
                   node_neighbors: &'d [Vec<usize>])
                   -> Layout<'c, 'd, V> {
        let n = node_positions.len();
        assert!(node_neighbors.len() == n);
        Layout {
            forces: (0..n).map(|_| V::new()).collect(), // initialize forces
            node_positions: node_positions,
            node_neighbors: node_neighbors,
            lock_first_n_positions: 0,
        }
    }

    fn lock_first_n_positions(&mut self, n: usize) {
        self.lock_first_n_positions = n;
    }
}

impl<'a, 'b, V> ForceDirected<V> for Layout<'a, 'b, V>
    where V: Vector<Scalar = f32>
{
    fn reset_forces(&mut self) {
        for f in self.forces.iter_mut() {
            f.reset();
        }
    }

    fn update_force_each_node_pair<F: Fn(&V, &V) -> V>(&mut self, f: F) {
        let n = self.node_positions.len();
        assert!(n == self.forces.len());

        for i1 in 0..n - 1 {
            for i2 in i1 + 1..n {
                let force = f(&self.node_positions[i1], &self.node_positions[i2]);
                self.forces[i1].add_scaled(1.0, &force);
                self.forces[i2].add_scaled(-1.0, &force);
            }
        }
    }

    fn update_force_each_edge<F: Fn(&V, &V) -> V>(&mut self, f: F) {
        let n = self.node_positions.len();
        assert!(n == self.forces.len());

        for i1 in 0..n {
            for &i2 in self.node_neighbors[i1].iter() {
                let force = f(&self.node_positions[i1], &self.node_positions[i2]);
                self.forces[i1].add_scaled(1.0, &force);
                self.forces[i2].add_scaled(-1.0, &force);
            }
        }
    }

    fn update_positions<F: FnMut(&V, &V) -> V>(&mut self, mut f: F) {
        let n = self.node_positions.len();
        assert!(n == self.forces.len());

        for i in self.lock_first_n_positions..n {
            let new_pos = f(&self.node_positions[i], &self.forces[i]);
            self.node_positions[i] = new_pos;
        }
    }
}

pub fn layout_typical_2d<'a, 'b>(l: Option<f32>,
                                 node_positions: &'a mut Vec<P2d>,
                                 node_neighbors: &'b [Vec<usize>],
                                 lock_first_n_positions: usize) {
    let n = node_positions.len();
    assert!(node_neighbors.len() == n);

    const MAX_ITER: usize = 300;
    const EPS: f32 = 0.01;

    let temp = 0.1f32;
    let dt = temp / (MAX_ITER as f32);
    let min_pos = P2d(0.0, 0.0);
    let max_pos = P2d(1.0, 1.0);
    let step_fn = |iter| temp - (iter as f32 * dt);

    // `l`: ideal length of spring
    let l: f32 = l.unwrap_or((1.0 / n as f32).sqrt());

    let k_r = l * l;
    let k_s = l;

    let mut lay = Layout::new(node_positions, node_neighbors);
    lay.lock_first_n_positions(lock_first_n_positions);
    layout(&mut lay,
           step_fn,
           MAX_ITER,
           EPS,
           k_r,
           k_s,
           &min_pos,
           &max_pos);
}
