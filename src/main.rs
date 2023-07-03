use audio_processor::AudioProcessor;
use graph::Graph;

mod audio_processor;
mod graph;

fn main() {
    let mixer = Mixer;
    let osc1 = Osc { phase: 0.0 };
    let osc2 = Osc { phase: 0.0 };

    let mut graph: Graph<64> = Graph::new();

    let mixer = graph::Module::new(
        mixer,
        vec![("a".to_string(), 0.0), ("b".to_string(), 0.0)],
        vec![("out".to_string())],
        &mut graph,
    );

    let osc1 = graph::Module::new(
        osc1,
        vec![("fq".to_string(), 440.0)],
        vec!["out".to_string()],
        &mut graph,
    );
    let osc2 = graph::Module::new(
        osc2,
        vec![("fq".to_string(), 880.0)],
        vec!["out".to_string()],
        &mut graph,
    );

    let mixer_output = mixer.outputs()[0].1;

    graph
        .connections
        .insert(mixer.inputs()[0].1, osc1.outputs()[0].1);
    graph
        .connections
        .insert(mixer.inputs()[1].1, osc2.outputs()[0].1);

    graph.nodes.insert(osc1);
    graph.nodes.insert(osc2);
    graph.nodes.insert(mixer);

    graph.process(2000.0);

    println!("{:?}", graph.outputs.get(mixer_output));
}

struct Mixer;

impl<const N: usize> AudioProcessor<N> for Mixer {
    fn n_inputs(&self) -> usize {
        2
    }

    fn n_outputs(&self) -> usize {
        1
    }

    fn process(&mut self, inputs: &[[f32; N]], outputs: &mut [[f32; N]], _: f32) {
        for i in 0..N {
            outputs[0][i] = (inputs[0][i] + inputs[1][i]) / 2.0
        }
    }
}

struct Osc {
    phase: f32,
}

impl<const N: usize> AudioProcessor<N> for Osc {
    fn n_inputs(&self) -> usize {
        1
    }

    fn n_outputs(&self) -> usize {
        1
    }

    fn process(&mut self, inputs: &[[f32; N]], outputs: &mut [[f32; N]], sr: f32) {
        for i in 0..N {
            outputs[0][i] = (self.phase * std::f32::consts::TAU).sin();

            self.phase = (self.phase + inputs[0][i] / sr).fract();
        }
    }
}
