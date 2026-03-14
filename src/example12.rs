use egui::{load::SizedTexture, TextureId, Vec2, ViewportId};
use glium::{
    backend::glutin::SimpleWindowBuilder, glutin::surface::WindowSurface, implement_vertex,
    index::PrimitiveType, uniform, Display, IndexBuffer, Program, Surface, VertexBuffer,
};
use nalgebra::{Matrix4, Point3, Vector3};
use obj::{load_obj, Obj};
use std::fs::File;
use std::io::BufReader;
use std::time::Instant;
use winit::{
    application::ApplicationHandler,
    event::{StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[derive(Copy, Clone, Debug)]
pub struct Vertex {
    /// Position vector of a vertex.
    pub position: [f32; 3],
    /// Normal vertor of a vertex.
    pub normal: [f32; 3],
}
implement_vertex!(Vertex, position, normal);

impl From<&obj::Vertex> for Vertex {
    fn from(vertex: &obj::Vertex) -> Self {
        Vertex {
            position: vertex.position,
            normal: vertex.normal,
        }
    }
}

impl From<obj::Vertex> for Vertex {
    fn from(vertex: obj::Vertex) -> Self {
        Vertex {
            position: vertex.position,
            normal: vertex.normal,
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Winit program
    let event_loop = EventLoop::new().unwrap();
    let (window, display) = SimpleWindowBuilder::new()
        .set_window_builder(Window::default_attributes().with_resizable(true))
        .with_inner_size(800, 600)
        .with_title("egui_glium 3D Teapot load obj example")
        .build(&event_loop);

    event_loop.set_control_flow(ControlFlow::Poll);

    // 3D glium stuff
    let input = BufReader::new(File::open("assets/teapot.obj")?);
    let model: Obj = load_obj(input)?;

    let vertex_data: Vec<Vertex> = model.vertices.iter().map(|v| v.into()).collect();
    let vertex_normals_buffer = VertexBuffer::new(&display, &vertex_data).unwrap();
    //let vertex_normals_buffer = model.vertex_buffer(&display)?;
    let index_buffer =
        IndexBuffer::new(&display, PrimitiveType::TrianglesList, &model.indices).unwrap();
    //let index_buffer = model.index_buffer(&display)?;

    // 3. Shaders (simple per-vertex coloring)
    let vertex_shader = r#"
#version 450
in vec3 position;
in vec3 normal;
uniform float uTime;
out vec3 v_color;
out vec3 v_normal;
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

    v_color = vec3(1.0, 0.0, 0.0);
    gl_Position = perspective * view * model * vec4(pos_rot, 1.0);
    v_normal = transpose(inverse(mat3(perspective * view * model))) * normal;
}
"#;

    let fragment_shader = r#"
#version 450
in vec3 v_color;
in vec3 v_normal;
out vec4 color;
uniform vec3 u_light;

void main() {
    float brightness = dot(normalize(v_normal), normalize(u_light));
    vec3 dark_color = vec3(0.6, 0.0, 0.0);
    vec3 regular_color = vec3(1.0, 0.0, 0.0);
    color = vec4(mix(dark_color, regular_color, brightness), 1.0);
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
        &Point3::new(0.0, 0.0, -200.0), // camera position
        &Point3::new(0.0, 0.0, 0.0),    // look-at point
        &Vector3::new(0.0, 1.0, 0.0),   // up vector
    );

    let mut egui_glium =
        egui_glium::EguiGlium::new(ViewportId::ROOT, &display, &window, &event_loop);

    let mut app = TeapotApp::new(
        egui_glium,
        window,
        display,
        Instant::now(),
        perspective,
        view,
        vertex_normals_buffer,
        index_buffer,
        program,
    );

    Ok(event_loop.run_app(&mut app)?)
}

fn create_display(
    event_loop: &EventLoop<()>,
) -> (winit::window::Window, glium::Display<WindowSurface>) {
    SimpleWindowBuilder::new()
        .set_window_builder(Window::default_attributes().with_resizable(true))
        .with_inner_size(800, 600)
        .with_title("egui_glium example")
        .build(event_loop)
}

struct TeapotApp {
    egui_glium: egui_glium::EguiGlium,
    window: winit::window::Window,
    display: glium::Display<WindowSurface>,
    close_requested: bool,
    start_time: Instant,
    perspective: Matrix4<f32>,
    view: Matrix4<f32>,
    vertex_normal_buffer: VertexBuffer<Vertex>,
    index_buffer: IndexBuffer<u16>,
    program: Program,
    last_x: f32,
    last_y: f32,
    rotate_x: f32,
    rotate_y: f32,
}

impl TeapotApp {
    pub fn new(
        egui_glium: egui_glium::EguiGlium,
        window: winit::window::Window,
        display: glium::Display<WindowSurface>,
        start_time: Instant,
        perspective: Matrix4<f32>,
        view: Matrix4<f32>,
        vertex_normal_buffer: VertexBuffer<Vertex>,
        index_buffer: IndexBuffer<u16>,
        program: Program,
    ) -> Self {
        TeapotApp {
            egui_glium,
            window,
            display,
            close_requested: false,
            start_time,
            perspective,
            view,
            vertex_normal_buffer,
            index_buffer,
            program,
            last_x: 0.0,
            last_y: 0.0,
            rotate_x: 0.0,
            rotate_y: 0.0,
        }
    }
}

impl ApplicationHandler for TeapotApp {
    fn resumed(&mut self, _event_loop: &ActiveEventLoop) {
        println!("resumed");
        //self.window = match event_loop.create_window(Window::default_attributes()) {
        //    Ok(window) => window,
        //    Err(err) => {
        //        eprintln!("error creating window: {err}");
        //        event_loop.exit();
        //        return;
        //    }
        //};
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let mut redraw = || {
            let mut quit = false;
            self.egui_glium.run(&self.window, |egui_ctx| {
                egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                    if ui.add(egui::Button::new("close")).clicked() {
                        println!("close button clicked");
                        self.close_requested = true;
                    };
                });
                egui::Window::new("TeapotDisplay").show(egui_ctx, |ui| {
                    // show sth
                });
            });

            {
                // Rotate cube
                let model = Matrix4::from_axis_angle(&Vector3::x_axis(), self.rotate_y)
                    * Matrix4::from_axis_angle(&Vector3::y_axis(), self.rotate_x);

                // TODO: make light moveing by parameter
                let light = [-1.0, 0.4, 0.9f32];

                // Uniforms for the shader
                let uniforms = uniform! {
                    perspective: Into::<[[f32; 4]; 4]>::into(self.perspective),
                    view: Into::<[[f32; 4]; 4]>::into(self.view),
                    model: Into::<[[f32; 4]; 4]>::into(model),
                    uTime: elapsed,
                    u_light: light,
                };

                // Do Glium rendering:
                let mut target = self.display.draw();
                target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

                // Draw the cube
                let params = glium::DrawParameters {
                    depth: glium::Depth {
                        test: glium::draw_parameters::DepthTest::IfLess,
                        write: true,
                        ..Default::default()
                    },
                    ..Default::default()
                };
                target
                    .draw(
                        &self.vertex_normal_buffer,
                        &self.index_buffer,
                        &self.program,
                        &uniforms,
                        &params,
                    )
                    .unwrap();

                // draw things behind egui here

                self.egui_glium.paint(&self.display, &mut target);

                // draw things on top of egui here

                target.finish().unwrap();
            }
        };
        match &event {
            WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                println!("close event");
                self.close_requested = true;
            }
            WindowEvent::Resized(new_size) => {
                self.display.resize((*new_size).into());
            }
            WindowEvent::RedrawRequested => redraw(),
            _ => {}
        }

        let event_response = self.egui_glium.on_event(&self.window, &event);

        if event_response.repaint {
            self.window.request_redraw();
        }
        if self.close_requested {
            println!("close requested; stopping");
            event_loop.exit();
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, cause: StartCause) {
        if let StartCause::ResumeTimeReached { .. } = cause {
            self.window.request_redraw();
        }
    }
}
