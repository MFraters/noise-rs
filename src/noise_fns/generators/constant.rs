use crate::{noisefield::NoiseField2D, NoiseFieldFn, NoiseFn};

/// Noise function that outputs a constant value.
///
/// This function takes a input, value, and returns that input for all points,
/// producing a constant-valued field.
///
/// This function is not very useful by itself, but can be used as a source
/// function for other noise functions.
#[derive(Clone, Copy, Debug)]
pub struct Constant {
    /// Constant value.
    pub value: f64,
}

impl Constant {
    pub fn new(value: f64) -> Self {
        Self { value }
    }
}

impl<T: Copy> NoiseFn<T> for Constant {
    fn get(&self, _point: T) -> f64 {
        self.value
    }
}

impl NoiseFieldFn<NoiseField2D> for Constant {
    fn process_field(&self, field: &NoiseField2D) -> NoiseField2D {
        let mut output = field.clone();

        for i in 0..output.values.len() {
            output.values[i] = self.value
        }

        output
    }
}
