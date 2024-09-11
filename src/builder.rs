use std::{collections::HashMap, hash::Hash};

use crate::{App, LayerState, LayerType, Pixu, PixuResult};

pub struct PixuBuilder<Layer> {
    pub(crate) title: String,
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) layers: HashMap<Layer, LayerType>,
    pub(crate) app: Box<dyn App<Layer>>,
}

impl<Layer> PixuBuilder<Layer>
where
    Layer: Eq + Hash + Copy,
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

    fn build(self) -> Pixu<Layer> {
        let layers = self
            .layers
            .iter()
            .map(|(layer, layer_type)| {
                let (width, height, scale) = match *layer_type {
                    LayerType::ScaledLayer(scale) => {
                        (self.width / scale, self.height / scale, scale)
                    }
                };

                (
                    *layer,
                    LayerState {
                        scale,
                        width,
                        height,
                        pixels: vec![0; (width * height) as usize],
                    },
                )
            })
            .collect::<HashMap<Layer, LayerState>>();

        Pixu {
            title: self.title,
            width: self.width,
            height: self.height,
            layers,
            window: None,
            render_state: Default::default(),
            tick_state: Default::default(),
            app: self.app,
            exiting: false,
        }
    }

    pub fn run(self) -> PixuResult<()> {
        let mut pixu = self.build();
        pixu.run()
    }
}
