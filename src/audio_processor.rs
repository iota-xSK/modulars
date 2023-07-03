pub trait AudioProcessor<const N: usize> {
    fn n_inputs(&self) -> usize;
    fn n_outputs(&self) -> usize;

    fn input_name(&self, idx: usize) -> String;
    fn output_name(&self, idx: usize) -> String;

    fn process(&mut self, inputs: &[[f32; N]], outputs: &mut [[f32; N]], sr: f32);
}
