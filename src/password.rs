use std::io;
use std::io::Write;

extern crate rpassword;

pub fn get_password() -> String {
    print!("Password: ");
    io::stdout().flush().unwrap();
    rpassword::read_password().unwrap()
}
