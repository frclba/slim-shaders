use gl::types::*;
use std::mem;
use std::os::raw::*;

pub struct Vao {
    id: gl::types::GLuint,
}

impl Default for Vao {
    fn default() -> Self {
        Self::new()
    }
}

impl Vao {
    pub fn new() -> Vao {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut id);
        }
        Vao { id }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindVertexArray(self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindVertexArray(0);
        }
    }
}

pub struct BufferObject {
    id: gl::types::GLuint,
    r#type: gl::types::GLenum,
    usage: gl::types::GLenum,
}

impl BufferObject {
    pub fn new(r#type: gl::types::GLenum, usage: gl::types::GLenum) -> BufferObject {
        let mut id: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut id);
        }
        BufferObject { id, r#type, usage }
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, self.id);
        }
    }
    pub fn unbind(&self) {
        unsafe {
            gl::BindBuffer(self.r#type, 0);
        }
    }

    pub fn store_f32_data(&self, data: &[f32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                std::mem::size_of_val(data) as GLsizeiptr,
                &data[0] as *const f32 as *const GLvoid,
                self.usage,
            );
        }
    }
    pub fn store_i32_data(&self, data: &[i32]) {
        unsafe {
            gl::BufferData(
                self.r#type,
                (data.len() * mem::size_of::<GLfloat>()) as GLsizeiptr,
                &data[0] as *const i32 as *const GLvoid,
                self.usage,
            );
        }
    }
}

pub struct VertexAttribute {
    index: GLuint,
}

impl VertexAttribute {
    /// Creates a new `VertexAttribute`.
    ///
    /// # Safety
    /// This function is unsafe because it directly interacts with raw pointers and OpenGL functions.
    ///
    /// Parameters:
    /// - `index`: The index of the vertex attribute.
    /// - `size`: The number of components per attribute.
    /// - `r#type`: The data type of each component.
    /// - `normalized`: Specifies whether the attribute values should be normalized.
    /// - `stride`: The byte offset between consecutive attributes.
    /// - `pointer`: The pointer to the first component of the first attribute.
    ///
    /// Returns:
    /// The created `VertexAttribute`.
    pub unsafe fn new(
        index: u32,
        size: i32,
        r#type: GLenum,
        normalized: GLboolean,
        stride: GLsizei,
        pointer: *const c_void,
    ) -> VertexAttribute {
        unsafe {
            gl::VertexAttribPointer(index, size, r#type, normalized, stride, pointer);
        }
        VertexAttribute { index }
    }

    pub fn enable(&self) {
        unsafe {
            gl::EnableVertexAttribArray(self.index);
        }
    }

    pub fn disable(&self) {
        unsafe {
            gl::DisableVertexAttribArray(self.index);
        }
    }
}
