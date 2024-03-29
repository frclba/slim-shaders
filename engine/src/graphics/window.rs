use glfw::GlfwReceiver;
use glfw::{Action, Context, Key, WindowEvent};

pub struct Window {
    glfw: glfw::Glfw,
    window_handle: glfw::PWindow,
    events: GlfwReceiver<(f64, WindowEvent)>, // Change the type here
}

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Window {
        let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
        let (mut window, events) = glfw
            .create_window(width, height, title, glfw::WindowMode::Windowed)
            .expect("Failed to create GLFW window.");

        window.set_framebuffer_size_polling(true);
        window.set_key_polling(true);

        Window {
            glfw,
            window_handle: window,
            events,
        }
    }

    pub fn init_gl(&mut self) {
        self.window_handle.make_current();
        self.window_handle.set_key_polling(true);
        gl::load_with(|symbol| self.window_handle.get_proc_address(symbol) as *const _);
    }

    pub fn should_close(&self) -> bool {
        self.window_handle.should_close()
    }

    pub fn update(&mut self) {
        self.process_events();
        self.glfw.poll_events();
        self.window_handle.swap_buffers();
    }

    fn process_events(&mut self) {
        for (_, even) in glfw::flush_messages(&self.events) {
            match even {
                WindowEvent::FramebufferSize(width, height) => unsafe {
                    gl::Viewport(0, 0, width, height);
                },
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    self.window_handle.set_should_close(true);
                }
                _ => {}
            }
        }
    }
}
