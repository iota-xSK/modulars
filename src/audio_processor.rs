pub trait AudioProcessor<const N: usize> {
    fn n_inputs(&self) -> usize;
    fn n_outputs(&self) -> usize;

    fn process(&mut self, inputs: &[[f32; N]], outputs: &mut [[f32; N]], sr: f32);
}
