use engine::graphics::gl_wrapper::*;
use engine::graphics::window::Window;

use engine::logger;
use gl::types::*;

use std::{mem, ptr};

fn main() {
    logger::init();

    let mut window = Window::new(1080, 720, "Hello");
    window.init_gl();

    let rectangle_vertices: [f32; 12] = [
        0.2, 0.2, 0.0, 0.2, -0.2, 0.0, -0.2, -0.2, 0.0, -0.2, 0.2, 0.0,
    ];

    let rectangle_indices: [i32; 6] = [0, 1, 3, 1, 2, 3];

    let triangle_vertices: [f32; 9] = [
        -0.5, 0.5, 0.0, // Left
        0.5, 0.5, 0.0, // Right
        0.0, -0.5, 0.0, // Top
    ];

    let rectangle_vao = create_vao(&rectangle_vertices, Some(&rectangle_indices));

    create_vertex_attribute(1, 3);
    create_vertex_attribute(2, 3);

    let triangle_vao = create_vao(&triangle_vertices, None);
    create_vertex_attribute(0, 3);

    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.1, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        render_shapes(&rectangle_vao, true);
        render_shapes(&triangle_vao, false);

        window.update();
    }
}

fn create_vao(vertices: &[f32], indices: Option<&[i32]>) -> engine::graphics::gl_wrapper::Vao {
    let vao = engine::graphics::gl_wrapper::Vao::new();
    vao.bind();

    let vbo = BufferObject::new(gl::ARRAY_BUFFER, gl::STATIC_DRAW);
    vbo.bind();
    vbo.store_f32_data(vertices);

    if let Some(indices) = indices {
        let ibo = BufferObject::new(gl::ELEMENT_ARRAY_BUFFER, gl::STATIC_DRAW);
        ibo.bind();
        ibo.store_i32_data(indices);
    }

    vao
}

fn create_vertex_attribute(index: u32, size: i32) {
    unsafe {
        let attribute = VertexAttribute::new(
            index,
            size,
            gl::FLOAT,
            gl::FALSE,
            3 * mem::size_of::<GLfloat>() as GLsizei,
            ptr::null(),
        );
        attribute.enable();
    }
}

fn render_shapes(vao: &engine::graphics::gl_wrapper::Vao, has_ibo: bool) {
    vao.bind();

    unsafe {
        if has_ibo {
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, ptr::null());
            return;
        }
        gl::DrawArrays(gl::TRIANGLES, 0, 3);
    }
}
