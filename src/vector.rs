pub trait Vector: Clone {
    type Scalar: Copy;

    fn length_squared(&self) -> Self::Scalar;

    fn scale(&mut self, factor: Self::Scalar);

    fn sub(&self, other: &Self) -> Self;

    fn add_scaled(&mut self, factor: Self::Scalar, other: &Self);

    fn reset(&mut self);

    fn clip_within(&mut self, min: &Self, max: &Self);

    fn new() -> Self;
}

#[derive(Debug, Copy, Clone)]
pub struct P2d (pub f32, pub f32);

impl Vector for P2d {
    type Scalar = f32;

    fn new() -> Self {
        P2d (0.0, 0.0)
    }

    fn length_squared(&self) -> f32 {
        self.0 * self.0 + self.1 * self.1
    }

    fn reset(&mut self) {
        self.0 = 0.0;
        self.1 = 0.0;
    }

    fn scale(&mut self, factor: f32) {
        self.0 *= factor;
        self.1 *= factor;
    }

    fn sub(&self, other: &Self) -> Self {
        P2d (self.0 - other.0, self.1 - other.1)
    }

    fn add_scaled(&mut self, factor: f32, other: &P2d) {
        self.0 += factor * other.0;
        self.1 += factor * other.1;
    }

    fn clip_within(&mut self, min: &Self, max: &Self) {
        if self.0 < min.0 {
            self.0 = min.0
        } else if self.0 > max.0 {
            self.0 = max.0
        }

        if self.1 < min.1 {
            self.1 = min.1
        } else if self.1 > max.1 {
            self.1 = max.1
        }
    }
}
