use std::time::Instant;

use glium::{
    backend::glutin::SimpleWindowBuilder, implement_vertex, index::PrimitiveType, uniform, Display,
    IndexBuffer, Program, Surface, VertexBuffer,
};
use glutin::surface::WindowSurface;
use nalgebra::{Matrix4, Point3, Vector3};
use winit::{
    application::ApplicationHandler,
    event::{ElementState, KeyEvent, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

// Vertex definition (position + color)
#[derive(Copy, Clone, Debug)]
struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}
implement_vertex!(Vertex, position, color);

struct CubePose {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug)]
struct Glium3DApp {
    display: Display<WindowSurface>,
    close_requested: bool,
    window: Option<Window>,
    start_time: Instant,
    perspective: Matrix4<f32>,
    view: Matrix4<f32>,
    vertex_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: Program,
    last_x: f32,
    last_y: f32,
    rotate_x: f32,
    rotate_y: f32,
}

impl Glium3DApp {
    pub fn new(
        display: Display<WindowSurface>,
        start_time: Instant,
        perspective: Matrix4<f32>,
        view: Matrix4<f32>,
        vertex_buffer: VertexBuffer<Vertex>,
        index_buffer: IndexBuffer<u16>,
        program: Program,
    ) -> Self {
        Glium3DApp {
            display,
            close_requested: false,
            window: None,
            start_time,
            perspective,
            view,
            vertex_buffer,
            index_buffer,
            program,
            last_x: 0.0,
            last_y: 0.0,
            rotate_x: 0.0,
            rotate_y: 0.0,
        }
    }
}

impl ApplicationHandler for Glium3DApp {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        // Do event handling
        //        println!("{event:?}");
        let elapsed = self.start_time.elapsed().as_secs_f32();
        match event {
            WindowEvent::CloseRequested => {
                self.close_requested = true;
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        logical_key: key,
                        state: ElementState::Pressed,
                        ..
                    },
                ..
            } => match key.as_ref() {
                _ => {
                    println!("pressed key: {:?}\n", key);
                }
            },
            WindowEvent::RedrawRequested => {
                // Rotate cube
                let model = Matrix4::from_axis_angle(&Vector3::x_axis(), self.rotate_y)
                    * Matrix4::from_axis_angle(&Vector3::y_axis(), self.rotate_x);

                // Uniforms for the shader
                let uniforms = uniform! {
                    perspective: Into::<[[f32; 4]; 4]>::into(self.perspective),
                    view: Into::<[[f32; 4]; 4]>::into(self.view),
                    model: Into::<[[f32; 4]; 4]>::into(model),
                    uTime: elapsed,
                };

                // Do Glium rendering:
                let mut target = self.display.draw();
                target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                // Draw the cube
                target
                    .draw(
                        &self.vertex_buffer,
                        &self.index_buffer,
                        &self.program,
                        &uniforms,
                        &Default::default(),
                    )
                    .unwrap();
                target.finish().unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            WindowEvent::CursorMoved {
                device_id,
                position,
            } => {
                // CursorMoved { device_id: DeviceId(Wayland(DeviceId)), position: PhysicalPosition { x: 0.0, y: 1.0 } }
                self.rotate_x += (self.last_x - position.x as f32) / 100.0;
                self.rotate_y += (self.last_y - position.y as f32) / 100.0;
                self.last_x = position.x as f32;
                self.last_y = position.y as f32;
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
        self.window = match event_loop.create_window(Window::default_attributes()) {
            Ok(window) => Some(window),
            Err(err) => {
                eprintln!("error creating window: {err}");
                event_loop.exit();
                return;
            }
        };
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let center = CubePose {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let h: f32 = 1.0;

    let event_loop = EventLoop::new().unwrap();
    let (_window, display) = SimpleWindowBuilder::new()
        .set_window_builder(Window::default_attributes().with_resizable(true))
        .with_inner_size(800, 600)
        .with_title("egui_glium 3D Cube example")
        .build(&event_loop);

    // 2. Define cube geometry (8 vertices, 12 triangles)
    let vertex_data = [
        // Front face (red -> green -> blue -> yellow)
        Vertex {
            position: [center.x - h, center.y - h, center.z + h],
            color: [0.0, 0.0, 0.0],
        },
        Vertex {
            position: [center.x + h, center.y - h, center.z + h],
            color: [0.0, 1.0, 0.0],
        },
        Vertex {
            position: [center.x + h, center.y + h, center.z + h],
            color: [0.0, 0.0, 1.0],
        },
        Vertex {
            position: [center.x - h, center.y + h, center.z + h],
            color: [1.0, 0.0, 0.0],
        },
        // Back face
        Vertex {
            position: [center.x - h, center.y - h, center.z - h],
            color: [1.0, 0.0, 1.0],
        },
        Vertex {
            position: [center.x + h, center.y - h, center.z - h],
            color: [0.0, 1.0, 1.0],
        },
        Vertex {
            position: [center.x + h, center.y + h, center.z - h],
            color: [1.0, 1.0, 0.0],
        },
        Vertex {
            position: [center.x - h, center.y + h, center.z - h],
            color: [1.0, 1.0, 1.0],
        },
    ];

    let index_data: &[u16] = &[
        0, 1, 2,
        2, 3, 0, // front
        5, 7, 4,
        7, 5, 6, // back
        1, 6, 5,
        6, 1, 2, // right
        4, 0, 3,
        3, 7, 4, // left
        3, 2, 6,
        7, 3, 6, // top
        4, 5, 1,
        1, 0, 4, // bottom
    ];

    let vertex_buffer = VertexBuffer::new(&display, &vertex_data).unwrap();
    let index_buffer =
        IndexBuffer::new(&display, PrimitiveType::TrianglesList, index_data).unwrap();

    // 3. Shaders (simple per-vertex coloring)
    let vertex_shader = r#"
#version 450
in vec3 position;
in vec3 color;
uniform float uTime;
out vec3 v_color;
uniform mat4 perspective;
uniform mat4 view;
uniform mat4 model;

mat2 rotate2d(float angle) {
    float s = sin(angle);
    float c = cos(angle);
    return mat2(c, -s, s, c);
}

void main() {
    vec3 pos_rot = position;
    pos_rot.yz = pos_rot.yz * rotate2d(uTime * 0.5);
    pos_rot.xz = pos_rot.xz * rotate2d(uTime);

    v_color = color;
    gl_Position = perspective * view * model * vec4(pos_rot, 1.0);
}
"#;

    let fragment_shader = r#"
#version 450
in vec3 v_color;
out vec4 color;
void main() {
    color = vec4(v_color, 1.0);
}
"#;

    let program = Program::from_source(&display, vertex_shader, fragment_shader, None).unwrap();

    // 4. Camera matrices
    let perspective = {
        let (width, height) = display.get_framebuffer_dimensions();
        let aspect_ratio = width as f32 / height as f32; // corrected
        let fov: f32 = std::f32::consts::PI / 3.0;
        let znear = 0.1;
        let zfar = 1024.0;
        Matrix4::new_perspective(aspect_ratio, fov, znear, zfar)
    };

    let view = Matrix4::look_at_rh(
        &Point3::new(0.0, 0.0, -10.0), // camera position
        &Point3::new(0.0, 0.0, 0.0),   // look-at point
        &Vector3::new(0.0, 1.0, 0.0),  // up vector
    );

    event_loop.set_control_flow(ControlFlow::Poll);
    let mut glium_app = Glium3DApp::new(
        display,
        Instant::now(),
        perspective,
        view,
        vertex_buffer,
        index_buffer,
        program,
    );
    event_loop.run_app(&mut glium_app)?;
    Ok(())
}
