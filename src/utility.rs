use std::process;

pub fn exit(code: i32, message: &str) {
    println!("{}", message);
    process::exit(code);
}
