///
/// Implements the force-directed graph layout algorithm as
/// proposed by Fruchterman and Reingold [1].
///
/// [1]: http://emr.cs.iit.edu/~reingold/force-directed.pdf
///

use super::{Vector, P2d};

// k_s == l
#[inline]
fn attractive_force<V>(p1: &V, p2: &V, k_s: f32) -> V
    where V: Vector<Scalar = f32>
{
    let mut force = p1.sub(p2);
    let length = force.length_squared().sqrt();
    let strength = length / k_s;
    force.scale(strength);
    return force;
}

// k_r == l^2
#[inline]
fn repulsive_force<V>(p1: &V, p2: &V, k_r: f32) -> V
    where V: Vector<Scalar = f32>
{
    let mut force = p1.sub(p2);
    let length_squared = force.length_squared();
    if length_squared > 0.0 {
        let strength = k_r / length_squared;
        force.scale(strength);
    }
    return force;
}

fn calculate_node_forces<V>(forces: &mut [V],
                            node_positions: &[V],
                            node_neighbors: &[Vec<usize>],
                            k_r: f32,
                            k_s: f32)
    where V: Vector<Scalar = f32>
{
    let n = forces.len();
    assert!(node_positions.len() == n);
    assert!(node_neighbors.len() == n);

    // Reset all forces to zero.
    for force in &mut forces[..] {
        force.reset();
    }

    // Calculate repulsive force between all pairs.
    for i1 in 0..n - 1 {
        for i2 in i1 + 1..n {
            let force = repulsive_force(&node_positions[i1], &node_positions[i2], k_r);
            forces[i1].add_scaled(1.0, &force);
            forces[i2].add_scaled(-1.0, &force);
        }
    }

    // Calculate spring force between adjacent pairs.
    for i1 in 0..n {
        for i2 in node_neighbors[i1].iter().map(|&i| i as usize) {
            let force = attractive_force(&node_positions[i1], &node_positions[i2], k_s);
            forces[i1].add_scaled(-1.0, &force);
            forces[i2].add_scaled(1.0, &force);
        }
    }
}

fn update_node_positions<V>(forces: &[V],
                            node_positions: &mut [V],
                            step: f32,
                            min_pos: &V,
                            max_pos: &V)
                            -> f32
    where V: Vector<Scalar = f32>
{
    let n = forces.len();
    assert!(node_positions.len() == n);

    let mut sum_distance = 0.0;

    for i in 0..n {
        let before = node_positions[i].clone();

        let length = forces[i].length_squared().sqrt();
        if length > 0.0 {
            // XXX: can we treat the scaled force as moved distance?
            node_positions[i].add_scaled(step / length, &forces[i]);
        }
        node_positions[i].clip_within(min_pos, max_pos);

        // add up the moved distance
        sum_distance += before.sub(&node_positions[i]).length_squared().sqrt();
    }

    return sum_distance;
}

pub fn layout<F, V>(step_fn: F,
                    max_iter: usize,
                    converge_eps: f32,
                    k_r: f32,
                    k_s: f32,
                    min_pos: &V,
                    max_pos: &V,
                    node_positions: &mut [V],
                    node_neighbors: &[Vec<usize>])
    where V: Vector<Scalar = f32>,
          F: Fn(usize) -> f32
{
    let n = node_positions.len();
    assert!(node_neighbors.len() == n);

    // initialize forces
    let mut forces: Vec<V> = (0..n).map(|_| V::new()).collect();

    let mut iter: usize = 0;
    while iter < max_iter {
        let step = step_fn(iter);
        iter += 1;

        calculate_node_forces(&mut forces, node_positions, node_neighbors, k_r, k_s);
        let dist_moved = update_node_positions(&forces, node_positions, step, min_pos, max_pos);
        if dist_moved < converge_eps {
            break;
        }
    }
}

pub fn layout_typical_2d(l: Option<f32>,
                         node_positions: &mut [P2d],
                         node_neighbors: &[Vec<usize>]) {
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

    layout(step_fn,
           MAX_ITER,
           EPS,
           k_r,
           k_s,
           &min_pos,
           &max_pos,
           node_positions,
           node_neighbors);
}
