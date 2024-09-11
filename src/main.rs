use pixu::{App, Pixu, PixuResult, RenderState, TickState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Layer {
    Emulator,
    Ui,
}

fn main() -> PixuResult<()> {
    Pixu::build(Box::new(TestApp::new()))
        .with_title("Hello, World!")
        .with_size(800, 600)
        .with_scaled_layer(Layer::Emulator, 4)
        .with_scaled_layer(Layer::Ui, 2)
        .run()
}

struct TestApp;

impl TestApp {
    fn new() -> Self {
        TestApp
    }
}

impl App<Layer> for TestApp {
    fn tick(&mut self, _state: TickState) {
        println!("Tick");
    }

    fn render(&self, state: RenderState, _layer: Layer, _pixels: &mut [u32]) {
        println!("Render");
    }
}
