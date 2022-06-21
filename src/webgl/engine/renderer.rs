use super::EngineContext;

use layer::Layer;

pub mod layer;

pub struct Renderer {
    context: EngineContext,

    layers: Vec<Layer>,
}

impl Renderer {
    pub fn new(context: EngineContext) -> Self {
        Self {
            context,
            layers: Vec::new(),
        }
    }

    pub fn add_layer(&mut self, layer: Layer) {
        self.layers.push(layer);
    }

    pub fn insert_layer(&mut self, index: usize, layer: Layer) {
        self.layers.insert(index, layer);
    }

    pub fn remove_layer(&mut self, index: usize) {
        self.layers.remove(index);
    }
}
