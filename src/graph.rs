use slotmap::new_key_type;

use crate::audio_processor::AudioProcessor;

new_key_type! { pub struct ModuleId; }
new_key_type! { pub struct ModuleInputId; }
new_key_type! { pub struct ModuleOutputId; }
pub struct Module<const N: usize> {
    processor: Box<dyn AudioProcessor<N>>,
    inputs: Vec<(String, ModuleInputId)>,
    outputs: Vec<(String, ModuleOutputId)>,
}

impl<const N: usize> Module<N> {
    fn process(&mut self, graph: &mut Graph<N>, sr: f32) {
        let mut inputs = vec![];

        for (_, input) in &self.inputs {
            if let Some(input_id) = graph.connections.get(*input) {
                inputs.push(graph.outputs[*input_id])
            } else {
                inputs.push([graph.inputs[*input]; N])
            }
        }

        let mut outputs = vec![[0.0; N]; self.processor.n_outputs()];

        self.processor.process(&inputs, &mut outputs, sr);

        for ((_, output), data) in self.outputs.iter().zip(outputs) {
            graph.outputs[*output] = data
        }
    }

    fn new(
        processor: impl AudioProcessor<N>,
        inputs: Vec<(String, f32)>,
        outputs: Vec<String>,
        graph: &mut Graph<N>,
    ) -> Self {
        let mut r = Self {
            processor: Box::new(processor),
            inputs: vec![],
            outputs: vec![],
        };

        for (name, value) in inputs {
            r.inputs.push((name, graph.inputs.insert(value)))
        }

        for name in outputs {
            r.outputs.push((name, graph.outputs.insert([0.0; N])))
        }

        r
    }

    fn input_id(&self, idx: usize) -> Option<ModuleInputId> {
        if let Some((_, id)) = self.inputs.get(idx) {
            Some(id.clone())
        } else {
            None
        }
    }
    fn output_id(&self, idx: usize) -> Option<ModuleOutputId> {
        if let Some((_, id)) = self.outputs.get(idx) {
            Some(*id)
        } else {
            None
        }
    }

    pub fn inputs(&self) -> &[(String, ModuleInputId)] {
        self.inputs.as_ref()
    }

    pub fn outputs(&self) -> &[(String, ModuleOutputId)] {
        self.outputs.as_ref()
    }

    pub fn outputs_mut(&mut self) -> &mut Vec<(String, ModuleOutputId)> {
        &mut self.outputs
    }

    pub fn inputs_mut(&mut self) -> &mut Vec<(String, ModuleInputId)> {
        &mut self.inputs
    }

    pub fn processor(&self) -> &dyn AudioProcessor<N> {
        self.processor.as_ref()
    }

    pub fn processor_mut(&mut self) -> &mut Box<dyn AudioProcessor<N>> {
        &mut self.processor
    }
}

use slotmap::*;

pub struct Graph<const N: usize> {
    pub nodes: SlotMap<ModuleId, Module<N>>,
    pub inputs: SlotMap<ModuleInputId, f32>,
    pub outputs: SlotMap<ModuleOutputId, [f32; N]>,
    pub connections: SecondaryMap<ModuleInputId, ModuleOutputId>,
}

impl<const N: usize> Graph<N> {
    pub fn new() -> Self {
        Self {
            nodes: SlotMap::with_key(),
            inputs: SlotMap::with_key(),
            outputs: SlotMap::with_key(),
            connections: SecondaryMap::new(),
        }
    }

    pub fn process(&mut self, sr: f32) {
        for node in self.nodes.values_mut() {
            let mut inputs = vec![];

            for (_, input) in node.inputs() {
                if let Some(input_id) = self.connections.get(*input) {
                    inputs.push(self.outputs[*input_id])
                } else {
                    inputs.push([self.inputs[*input]; N])
                }
            }

            println!("{:?}", inputs);

            let mut outputs = vec![[0.0; N]; node.processor().n_outputs()];

            node.processor_mut().process(&inputs, &mut outputs, sr);

            for ((_, output), data) in node.outputs.iter().zip(outputs) {
                self.outputs[*output] = data
            }
        }
    }
}
