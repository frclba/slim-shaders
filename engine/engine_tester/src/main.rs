use engine::graphics::gl_wrapper::*;
use engine::graphics::window::Window;
use gl::types::*;

use engine::logger;
use std::mem;
use std::ptr;

fn main() {
    logger::init();
    let mut window = Window::new(800, 600, "Engine Tester");

    let vertices: [f32; 9] = [
        -0.5, -0.5, 0.0, // Left
        0.5, -0.5, 0.0, // Right
        0.0, 0.5, 0.0, // Top
    ];

    window.init_gl();

    let vao = engine::graphics::gl_wrapper::Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();

    vbo.store_f32_data(&vertices);
    unsafe {
        let position_attribute = VertexAttribute::new(
            0,
            3,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        position_attribute.enable();
    }

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.1, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::DrawArrays(gl::TRIANGLES, 0, 3);
        }
        window.update();
    }
}
