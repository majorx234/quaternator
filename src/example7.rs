use glium::{
    backend::glutin::SimpleWindowBuilder, implement_vertex, index::PrimitiveType, uniform,
    IndexBuffer, Program, Surface, VertexBuffer,
};
// use cgmath::{Matrix3, Matrix4, Vector3, Rad, Deg, InnerSpace, SquareMatrix, Transform, Vector4};
use nalgebra::{base::Matrix4, Point3, Vector3};
use std::time::Instant;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

// Vertex definition (position + color)
#[derive(Copy, Clone)]
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

// implement_vertex!(Vertex, position, color);

fn main() {
    let center = CubePose {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    let h: f32 = 1.0;
    // 1. Create window and OpenGL context
    let event_loop = EventLoop::new().unwrap();
    let (window, display) = SimpleWindowBuilder::new()
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
            color: [1.0, 1.0, 0.0],
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
            color: [0.5, 0.5, 0.5],
        },
        Vertex {
            position: [center.x - h, center.y + h, center.z - h],
            color: [1.0, 1.0, 1.0],
        },
    ];

    let index_data: &[u16] = &[
        0, 1, 2, 2, 3, 0, // front
        1, 5, 6, 6, 2, 1, // right
        5, 4, 7, 7, 6, 5, // back
        4, 0, 3, 3, 7, 4, // left
        3, 2, 6, 6, 7, 3, // top
        4, 5, 1, 1, 0, 4, // bottom
    ];

    let vertex_buffer = VertexBuffer::new(&display, &vertex_data).unwrap();
    let index_buffer =
        IndexBuffer::new(&display, PrimitiveType::TrianglesList, index_data).unwrap();

    // 3. Shaders (simple per-vertex coloring)
    let vertex_shader = r#"
    #version 140
    in vec3 position;
    in vec3 color;
    out vec3 v_color;
    uniform mat4 perspective;
    uniform mat4 view;
    uniform mat4 model;
    void main() {
    v_color = color;
    gl_Position = perspective * view * model * vec4(position, 1.0);
}
"#;

    let fragment_shader = r#"
#version 140
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

    let start_time = Instant::now();

    // 5. Main event loop
    event_loop.run(move |event, control_flow| {
        let mut target = display.draw();
        target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);

        // Rotate cube over time
        let elapsed = start_time.elapsed().as_secs_f32();
        let model = Matrix4::from_axis_angle(&Vector3::x_axis(), elapsed)
            * Matrix4::from_axis_angle(&Vector3::y_axis(), elapsed * 0.5);

        // Uniforms for the shader
        let uniforms = uniform! {
            perspective: Into::<[[f32; 4]; 4]>::into(perspective),
            view: Into::<[[f32; 4]; 4]>::into(view),
            model: Into::<[[f32; 4]; 4]>::into(model),
        };

        // Draw the cube
        target
            .draw(
                &vertex_buffer,
                &index_buffer,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();

        // Handle window close
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    control_flow.exit();
                }
                _ => (),
            },
            _ => (),
        }
    });
}
