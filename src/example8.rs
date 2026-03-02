use winit::{application::ApplicationHandler, event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, event::{Event, ElementState, KeyEvent, WindowEvent }, window::{Window, WindowAttributes, WindowId}};

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
        event_loop: &ActiveEventLoop,
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
            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            }
            _ => {
                println!("some event\n");
            }
        }
        if self.close_requested {
            println!("The close button was pressed; stopping");
            event_loop.exit();
        }
    }
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        println!("resumed");
        self.window = match event_loop.create_window(Window::default_attributes()){
            Ok(window) => Some(window),
            Err(err) => {
                eprintln!("error creating window: {err}");
                event_loop.exit();
                return;
            },
        };
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut Glium3DApp::default())?;
    Ok(())
}
