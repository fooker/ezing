#![cfg_attr(feature = "cargo-clippy", allow(clippy))]
#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate glutin;
extern crate ezing;

use glium_text::{FontTexture,TextSystem,TextDisplay,draw};

#[derive(Copy, Clone)]
struct Vertex {
  position: [f32; 2],
}
implement_vertex!(Vertex, position);

const FNS: [(&'static str, [fn(f32) -> f32; 3]); 10] = [
  ("quad", [ezing::quad_in, ezing::quad_out, ezing::quad_inout]),
  ("cubic", [ezing::cubic_in, ezing::cubic_out, ezing::cubic_inout]),
  ("quart", [ezing::quart_in, ezing::quart_out, ezing::quart_inout]),
  ("quint", [ezing::quint_in, ezing::quint_out, ezing::quint_inout]),
  ("sine", [ezing::sine_in, ezing::sine_out, ezing::sine_inout]),
  ("circ", [ezing::circ_in, ezing::circ_out, ezing::circ_inout]),
  ("expo", [ezing::expo_in, ezing::expo_out, ezing::expo_inout]),
  ("elastic", [ezing::elastic_in, ezing::elastic_out, ezing::elastic_inout]),
  ("back", [ezing::back_in, ezing::back_out, ezing::back_inout]),
  ("bounce", [ezing::bounce_in, ezing::bounce_out, ezing::bounce_inout]),
];


fn build_lines() -> Vec<Vertex> {
  let vpad = 0.02;
  let hpad = 0.02;
  let lpad = 0.1;
  let stroke = 0.015;

  let width = (1.0 - lpad - 4.0 * hpad) / 3.0;
  let height = (1.0 - ((FNS.len() + 1) as f32 * vpad)) / FNS.len() as f32;

  let mut vertices = vec![];

  for (i, row) in FNS.iter().enumerate() {
    let top = (height + vpad) * i as f32 + vpad;

    for (j, func) in row.1.iter().enumerate() {
      let left = (width + hpad) * j as f32 + hpad + lpad;

      let vertex = |x: f32, y: f32| -> Vertex {
        Vertex{ position: [x * width + left, (1.0 - y) * height + top] }
      };

      let n = 100;

      vertices.push(vertex(0.0, 0.0));
      vertices.push(vertex(0.0, 0.0));

      for i in 1..n {
        let x = i as f32 / n as f32;

        let y = func(x);
        vertices.push(vertex(x, y + stroke));
        vertices.push(vertex(x, y - stroke));
      }

      vertices.push(vertex(1.0, 1.0));
      vertices.push(vertex(1.0, 1.0));
    }
  }

  vertices
}

fn build_texts<'f>(text_system: &TextSystem,
                   font: &'f FontTexture)
                   -> Vec<(TextDisplay<&'f FontTexture>, [[f32; 4]; 4])> {
  let scale = 0.05;

  FNS.iter().enumerate().map(|(i, &(name, _))| {
    let text = TextDisplay::new(text_system, font, name);

    let y = i as f32 / FNS.len() as f32 * -2.0 + 1.0;

    let matrix = [
      [scale, 0.0, 0.0, 0.0],
      [0.0, scale, 0.0, 0.0],
      [0.0, 0.0, 1.0, 0.0],
      [-0.99, y - 0.17, 0.0, 1.0],
    ];

    (text, matrix)
  }).collect()
}

fn main() {
  // use glium::DisplayBuild;

  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new()
    .with_dimensions(1024, 768)
    .with_title("ezing demo");
  let context = glutin::ContextBuilder::new()
    .with_vsync(true)
    .with_multisampling(8);

  let display = glium::Display::new(window, context, &events_loop).unwrap();

  let program = glium::Program::from_source(&display, VERTEX_SHADER, FRAGMENT_SHADER, None).unwrap();

  let text_system = TextSystem::new(&display);
  let font_data: &[u8] = include_bytes!("Slabo27px-Regular.ttf");
  let font = FontTexture::new(&display, font_data, 70).unwrap();
  let black = (0.0, 0.0, 0.0, 1.0);

  let lines = build_lines();
  let vertex_buffer = glium::VertexBuffer::new(&display, &lines).unwrap();

  let texts = build_texts(&text_system, &font);

  loop {
    let mut closed = false;

    events_loop.poll_events(|event| {
      match event {
        glutin::Event::WindowEvent { event, .. } => match event {
          glutin::WindowEvent::Closed => { closed = true; }
          glutin::WindowEvent::KeyboardInput{input: glutin::KeyboardInput{ virtual_keycode:Some(glutin::VirtualKeyCode::Escape), .. }, .. } => { closed = true; }
          _ => (),
        },


        // glutin::Event::Closed
        _ => {},
      }
    });

    if closed { return; }

    let (w, h) = display.get_framebuffer_dimensions();
    let screen_ration = w as f32 / h as f32;

    {
      use glium::Surface;
      let mut target = display.draw();

      let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

      target.clear_color(1.0, 1.0, 1.0, 1.0);

      target.draw(&vertex_buffer,
                  &indices,
                  &program,
                  &glium::uniforms::EmptyUniforms,
                  &Default::default()).unwrap();

      for &(ref text, matrix) in &texts {
        let mut matrix = matrix.clone();
        matrix[1][1] *= screen_ration;

        draw(&text, &text_system, &mut target, matrix, black);
      }

      target.finish().unwrap();
    }
  }
}

const VERTEX_SHADER: &'static str = r#"
  #version 140

  in vec2 position;

  void main() {
    vec2 full = position * vec2(2, -2) + vec2(-1, 1);
    gl_Position = vec4(full, 0.0, 1.0);
  }
"#;

const FRAGMENT_SHADER: &'static str = r#"
  #version 140

  out vec4 color;

  void main() {
    color = vec4(0.0, 0.0, 0.0, 1.0);
  }
"#;
