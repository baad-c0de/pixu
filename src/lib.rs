mod builder;
mod error;

pub use error::*;

use std::collections::HashMap;

use builder::PixuBuilder;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

pub struct Pixu<Layer> {
    title: String,
    width: u32,
    height: u32,
    layers: HashMap<Layer, LayerType>,
    window: Option<Window>,
}

impl<Layer> Pixu<Layer> {
    pub fn build(app: Box<dyn App<Layer>>) -> PixuBuilder<Layer> {
        PixuBuilder {
            title: String::from("Pixu App"),
            width: 800,
            height: 600,
            layers: HashMap::new(),
            app,
        }
    }

    pub(crate) fn run(&mut self) -> PixuResult<()> {
        let event_loop = EventLoop::new()?;
        event_loop.set_control_flow(ControlFlow::Poll);

        event_loop.run_app(self)?;

        Ok(())
    }

    //     loop {
    //         app.tick(TickState { delta_time: 0.0 });

    //         for (layer, layer_type) in &self.layers {
    //             match layer_type {
    //                 LayerType::ScaledLayer(scale) => {
    //                     let layer_width = self.width / scale;
    //                     let layer_height = self.height / scale;
    //                     let mut layer_pixels = vec![0; (layer_width * layer_height) as usize];
    //                     app.render(
    //                         RenderState {
    //                             width: layer_width,
    //                             height: layer_height,
    //                         },
    //                         layer,
    //                         &mut layer_pixels,
    //                     );
    //                 }
    //             }
    //         }
    //     }
    // }
}

impl<Layer> ApplicationHandler for Pixu<Layer> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = WindowAttributes::default()
            .with_title(self.title.clone())
            .with_inner_size(PhysicalSize::new(self.width, self.height))
            .with_resizable(false);

        let window = event_loop.create_window(window_attributes).ok();
        self.window = window;
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(_) => {}
            WindowEvent::ScaleFactorChanged {
                scale_factor: _scale_factor,
                inner_size_writer: _inner_size_writer,
            } => {}
            WindowEvent::CloseRequested => {
                event_loop.exit();
            }
            WindowEvent::KeyboardInput {
                device_id: _device_id,
                event: _event,
                is_synthetic: _is_synthetic,
            } => {}
            WindowEvent::RedrawRequested => {}
            WindowEvent::ModifiersChanged(_) => {}

            _ => (),
        }
    }
}

enum LayerType {
    ScaledLayer(u32),
}

pub trait App<Layer> {
    fn tick(&mut self, state: TickState);
    fn render(&self, state: RenderState, layer: Layer, pixels: &mut [u32]);
}

pub struct TickState {
    pub delta_time: f64,
}

pub struct RenderState {
    pub width: u32,
    pub height: u32,
}
