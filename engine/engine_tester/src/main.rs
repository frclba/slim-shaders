use engine::custom_errors::Errors;
use engine::graphics::window::Window;
use engine::logger;

fn main() {
    logger::init();
    let _ = error_test(1);

    let mut window = Window::new(1080, 720, "Hello Window!");

    window.init_gl();
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.3, 0.9, 0.5, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.update();
    }
}

fn error_test(num: i32) -> Result<(), Errors> {
    if num == 1 {
        engine::logger::error!("Error test");
        return Err(Errors::TestError);
    }
    Ok(())
}
