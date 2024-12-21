use rand::Rng;
use std::iter::zip;

#[derive(Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

#[derive(Debug)]
struct Layer {
    neurons: Vec<Neuron>,
}

#[derive(Debug)]
struct Neuron {
    weights: Vec<f32>,
    bias: f32,
}

impl Network {
    pub fn prop(&self, mut inputs: Vec<f32>) -> Vec<f32> {
        for layer in &self.layers {
            inputs = layer.prop(inputs);
        }
        inputs
    }
    pub fn new(topology: &[usize]) -> Self {
        let layers = topology
            .windows(2)
            .map(|layers| Layer::random(layers[0], layers[1]))
            .collect();
        Self { layers }
    }
}

impl Layer {
    fn prop(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons.iter().map(|n| n.prop(&inputs)).collect()
    }
    fn random(current: usize, next: usize) -> Self {
        let neurons = (0..next).map(|_| Neuron::random(current)).collect();
        Self { neurons }
    }
}

impl Neuron {
    fn prop(&self, input: &[f32]) -> f32 {
        let output = zip(input, &self.weights).map(|(a, b)| a * b).sum::<f32>();
        (self.bias + output).max(0.0)
    }
    fn random(inputs: usize) -> Self {
        let mut rng = rand::thread_rng();
        let bias: f32 = rng.gen_range(-1.0..=1.0);
        let weights = (0..inputs).map(|_| rng.gen_range(-1.0..=1.0)).collect();
        Self { bias, weights }
    }
}
