pub trait Vector: Clone {
    type Scalar: Copy;

    fn length_squared(&self) -> Self::Scalar;

    fn scale(&self, factor: Self::Scalar) -> Self;

    fn sub(&self, other: &Self) -> Self;

    fn add_scaled(&mut self, factor: Self::Scalar, other: &Self);

    fn reset(&mut self);

    fn clip_within(&self, min: &Self, max: &Self) -> Self;

    fn new() -> Self;
}
