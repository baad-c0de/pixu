mod builder;
mod error;
mod state;

pub use error::*;
pub use state::*;

pub use winit::keyboard::{Key, PhysicalKey};

use tracing::trace;

use std::collections::HashMap;

use builder::PixuBuilder;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{ElementState, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

pub struct Pixu<Layer> {
    title: String,
    width: u32,
    height: u32,
    layers: HashMap<Layer, LayerState>,
    window: Option<Window>,
    render_state: RenderState,
    tick_state: TickState,
    app: Box<dyn App<Layer>>,
    exiting: bool,
}

struct LayerState {
    scale: u32,
    width: u32,
    height: u32,
    pixels: Vec<u32>,
}

impl<Layer> Pixu<Layer>
where
    Layer: Copy,
{
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

    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;

        // Adjust the layers
        self.layers.iter_mut().for_each(|(_, layer)| {
            layer.width = width / layer.scale;
            layer.height = height / layer.scale;
            layer.pixels = vec![0; (layer.width * layer.height) as usize];
        });
    }
}

impl<Layer> ApplicationHandler for Pixu<Layer>
where
    Layer: Copy,
{
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attributes = WindowAttributes::default()
                .with_title(self.title.clone())
                .with_inner_size(PhysicalSize::new(self.width, self.height))
                .with_resizable(false);

            let window = event_loop.create_window(window_attributes).ok();
            self.window = window;
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => {
                trace!("Resized: {:?}", size);
                self.resize(size.width, size.height);
            }
            WindowEvent::CloseRequested => {
                trace!("Close requested");
                event_loop.exit();
            }
            WindowEvent::KeyboardInput { event, .. } => {
                trace!("Keyboard input: {:?}", event);

                let physical_key = event.physical_key;
                let logical_key = match event.logical_key {
                    Key::Character(s) => {
                        if s.len() == 1 {
                            Some(s.chars().next().unwrap())
                        } else {
                            None
                        }
                    }
                    _ => None,
                };

                if event.state == ElementState::Pressed {
                    self.tick_state
                        .events
                        .push(TickEvent::KeyPressed(physical_key, logical_key));
                } else {
                    self.tick_state
                        .events
                        .push(TickEvent::KeyReleased(physical_key, logical_key));
                }
            }
            WindowEvent::RedrawRequested => {
                self.layers.iter_mut().for_each(|(layer, layer_state)| {
                    self.app.render(
                        &self.render_state,
                        *layer,
                        layer_state.width,
                        layer_state.height,
                        &mut layer_state.pixels,
                    )
                });
            }
            WindowEvent::ModifiersChanged(mods) => {
                trace!("Modifiers changed: {:?}", mods);
                self.tick_state
                    .events
                    .push(TickEvent::ModifiersChanged(mods));
            }

            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if !self.exiting {
            if self.app.tick(&self.tick_state) == PixuResponse::Exit {
                trace!("Exiting...");
                self.exiting = true;
                event_loop.exit();
            }
            if let Some(window) = &self.window {
                window.request_redraw();
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum LayerType {
    ScaledLayer(u32),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PixuResponse {
    Continue,
    Exit,
}

pub trait App<Layer> {
    fn tick(&mut self, state: &TickState) -> PixuResponse;
    fn render(
        &self,
        state: &RenderState,
        layer: Layer,
        layer_width: u32,
        layer_height: u32,
        pixels: &mut [u32],
    );
}
