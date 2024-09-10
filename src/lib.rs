use std::{collections::HashMap, hash::Hash};

pub struct Pixu<Layer> {
    title: String,
    width: u32,
    height: u32,
    layers: HashMap<Layer, LayerType>,
}

pub struct PixuBuilder<Layer> {
    title: String,
    width: u32,
    height: u32,
    layers: HashMap<Layer, LayerType>,
}

impl<Layer> Pixu<Layer> {
    pub fn build() -> PixuBuilder<Layer> {
        PixuBuilder {
            title: String::from("Pixu App"),
            width: 800,
            height: 600,
            layers: HashMap::new(),
        }
    }

    pub fn run(&self, mut app: impl App) {
        loop {
            app.tick(TickState { delta_time: 0.0 });

            for (layer, layer_type) in &self.layers {
                match layer_type {
                    LayerType::ScaledLayer(scale) => {
                        let layer_width = self.width / scale;
                        let layer_height = self.height / scale;
                        let mut layer_pixels = vec![0; (layer_width * layer_height) as usize];
                        app.render(
                            RenderState {
                                width: layer_width,
                                height: layer_height,
                            },
                            layer,
                            &mut layer_pixels,
                        );
                    }
                }
            }
        }
    }
}

enum LayerType {
    ScaledLayer(u32),
}

impl<Layer> PixuBuilder<Layer>
where
    Layer: Eq + Hash,
{
    pub fn with_title(self, title: &str) -> Self {
        PixuBuilder {
            title: title.to_string(),
            ..self
        }
    }

    pub fn with_size(self, width: u32, height: u32) -> Self {
        PixuBuilder {
            width,
            height,
            ..self
        }
    }

    pub fn with_scaled_layer(self, layer: Layer, scale: u32) -> Self {
        let mut layers = self.layers;
        layers.insert(layer, LayerType::ScaledLayer(scale));
        PixuBuilder { layers, ..self }
    }

    pub fn build(self) -> Pixu<Layer> {
        Pixu {
            title: self.title,
            width: self.width,
            height: self.height,
            layers: self.layers,
        }
    }
}

pub trait App {
    fn tick(&mut self, state: TickState);
    fn render<Layer>(&self, state: RenderState, layer: Layer, pixels: &mut [u32]);
}

pub struct TickState {
    pub delta_time: f64,
}

pub struct RenderState {
    pub width: u32,
    pub height: u32,
}
