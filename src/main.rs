extern crate glium;
extern crate glium_text;
extern crate cgmath;
mod text;

fn main() {
    use glium::DisplayBuild;
    let display = glium::glutin::WindowBuilder::new()
        .build_glium()
        .unwrap();

    let system = text::System::new(&display);
    let font_size = 12;
    let font = system.new_font(include_bytes!("DejaVuSerif.ttf"), font_size);

    let lines = include_str!("/home/joonazan/go/src/github.com/joonazan/vec2/vec2.go")
        .split("\n")
        .map(|x| font.new_text(x))
        .collect();

    let code = Code{
        lines: lines,
        height: font_size as f32,
    };

    loop {
        for ev in display.poll_events() {
            use glium::glutin::Event;
            match ev {
                Event::Closed => return,
                _ => (),
            }
        }

        let (w, h) = display.get_framebuffer_dimensions();

        use glium::Surface;
        let mut frame = display.draw();
        frame.clear_color(1., 1., 1., 1.);

        code.draw(&mut frame, w as f32, h as f32);

        frame.finish().unwrap();
    }
}

use std::ops;

struct Code <'a, T: ops::Deref<Target=glium_text::FontTexture>, F: text::SizedFacade + 'a> {
    lines: Vec<text::Text<'a, T, F>>,
    height: f32,
}

impl <'a, T: ops::Deref<Target=glium_text::FontTexture>, F: text::SizedFacade> Code <'a, T, F> {
    fn draw(&self, frame: &mut glium::Frame, display_width: f32, display_height: f32) {
        let mut mat = cgmath::Matrix4::from_nonuniform_scale(self.height / display_width * 2.0, self.height / display_height * 2.0, 1.0);

        for line in self.lines.iter() {
            line.draw(frame, mat.into(), (0., 0., 0., 1.));
            mat = mat * cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.0, -1.5, 0.0))
        }
    }
}

/*fn readFile() -> &str {
    let path = "/home/joonazan/go/src/github.com/joonazan/vec2/vec2.go"


}*/