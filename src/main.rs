use pixu::{App, Pixu, RenderState, TickState};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Layer {
    Emulator,
    Ui,
}

fn main() {
    let p = Pixu::build()
        .with_title("Hello, World!")
        .with_size(800, 600)
        .with_scaled_layer(Layer::Emulator, 4)
        .with_scaled_layer(Layer::Ui, 2)
        .build();

    let app = TestApp::new();

    p.run(app);
}

struct TestApp;

impl TestApp {
    fn new() -> Self {
        TestApp
    }
}

impl App for TestApp {
    fn tick(&mut self, _state: TickState) {
        println!("Tick");
    }

    fn render<Layer>(&self, state: RenderState, _layer: Layer, _pixels: &mut [u32]) {
        println!("Render");
    }
}
