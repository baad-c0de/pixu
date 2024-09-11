use pixu::{App, Pixu, PixuResponse, PixuResult, RenderState, TickEvent, TickState};
use tracing_subscriber::{fmt, EnvFilter};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Layer {
    Emulator,
    Ui,
}

fn main() -> PixuResult<()> {
    let subscriber = fmt::Subscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        // .pretty()
        .compact()
        .without_time()
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to initialize the logger.");

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
    fn tick(&mut self, state: &TickState) -> PixuResponse {
        let mut response = PixuResponse::Continue;
        state.events.iter().for_each(|event| {
            if let TickEvent::KeyPressed(_, Some('q')) = event {
                response = PixuResponse::Exit;
            }
        });
        response
    }

    fn render(
        &self,
        _state: &RenderState,
        _layer: Layer,
        _layer_width: u32,
        _layer_height: u32,
        _pixels: &mut [u32],
    ) {
    }
}
