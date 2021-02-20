use std::env;
use std::process;

mod uid;
mod config;
mod shadow;
mod utility;
mod password;

extern crate rpassword;
extern crate rscrypt;

fn main() {
    let args: Vec<String> = env::args().collect();

    let euid = uid::get_euid();

    if euid != 0 {
        utility::exit(1, "SUID root should be set on binary.");
    }

    let success = uid::set_uid(0);
    if success != 0 {
        utility::exit(1, "Unable to set uid");
    }

    let allowed_as_root = config::read_config();
    let current_user = config::get_user();
    let user_allowed = config::is_allowed(&current_user, allowed_as_root);

    if !user_allowed {
        utility::exit(1, "User is not permitted to run as root");
    }


    let password = password::get_password();

    let hash_string = shadow::get_user_hash(current_user);
    let salt = shadow::get_salt(hash_string.clone());
    let hash = rscrypt::c_crypt(&password, &salt);

    if hash != hash_string {
        utility::exit(1, "Incorrect password");
    }

    // Extract command name from arguments

    /*
    let mut command_name = String::new();
    if args.len() >= 2 {
        if false {
            println!("Running command {}", args[2]);
            command_name = args[2].clone();
        } else {
            command_name = args[1].clone();
        }
    }

    // Extract command args from arguments

    let mut command_args: Vec<String> = Vec::new();
    if args.len() > 2 {
        if false {
            command_args = args[3..].to_vec();
            println!("Args to command: {:?}", command_args);
        } else {
            command_args = args[2..].to_vec();
        }
    }
    */

    // Get shell
    let current_shell = env::var("SHELL").unwrap();

    let mut command: Vec<String> = Vec::new();
    command.push("-c".to_string());
    for arg in args[1..].into_iter() {
        command.push(arg.to_string());
    }

    // Execute command with args

    let mut cmd = process::Command::new(current_shell)
        .args(command)
        .spawn()
        .expect("Failed to spawn process");
    let _result = cmd
        .wait()
        .unwrap();
}
