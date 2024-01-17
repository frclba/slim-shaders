use engine::custom_errors::Errors;
use engine::logger;

fn main() {
    logger::init();
    let _ = error_test(1);
}

fn error_test(num: i32) -> Result<(), Errors> {
    if num == 1 {
        engine::logger::error!("Error test");
        return Err(Errors::TestError.into());
    }
    Ok(())
}
