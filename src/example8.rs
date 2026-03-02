use winit::{application::ApplicationHandler, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, event::{Event, ElementState, KeyEvent, WindowEvent }, window::{Window, WindowId}};


#[derive(Default, Debug)]
struct Glium3DApp {
    request_redraw: bool,
    wait_cancelled: bool,
    close_requested: bool,
    window: Option<Window>,
}

impl ApplicationHandler for Glium3DApp {
    fn window_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        println!("{event:?}");
        match event {
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            },
            WindowEvent::KeyboardInput {
                event: KeyEvent { logical_key: key, state: ElementState::Pressed, .. },
                ..
            } => match key.as_ref() {
                _ => {
                    println!("pressed key: {:?}\n",key);
                }
            },
            _ => {
                println!("some event\n");
            }
        }
    }
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    }
}

fn main() -> Result<(), impl std::error::Error> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.run_app(&mut Glium3DApp::default())
}
