use std::env;
use std::fs;
use std::process;

#[link(name = "c")]
extern "C" {
    fn geteuid() -> u32;
    fn setuid(_: u32) -> u32;
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut debug_mode = false;

    if args.len() >= 2 {
        if args[1] == "-d" {
            debug_mode = true;
        }
    }

    let mut euid: u32 = 666;
    unsafe {
        euid = geteuid();
    }

    if debug_mode {
        println!("UID of process: {}", euid);
    }

    if !debug_mode && euid != 0 {
        println!("'please' does not have root permissions, likely because SUID is not set.");
        println!("SUID root should be set on the binary.");
        process::exit(1);
    }

    unsafe {
        setuid(0);
        euid = geteuid();
    }

    if debug_mode {
        println!("UID of process [2]: {}", euid);
    }

    if debug_mode {
        //println!("Args: {:?}", args);
        println!("Accessing config file at /usr/local/etc/please.conf...");
    }
    
    let contents = fs::read_to_string("/usr/local/etc/please.conf");

    let contents = match contents {
        Ok(contents) => contents,
        Err(_) => {
            println!("Error opening config file at /usr/local/etc/please.conf");
            process::exit(1);
        },
    };

    if debug_mode {
        println!("Config contents: {}", contents);
    }

    let lines: Vec<&str> = contents
        .split("\n")
        .filter(|&line| line != " " && line != "")
        .collect();

    if debug_mode {
        println!("Lines in config: {:?}", lines);
    }

    let mut allowed_as_root: Vec<&str> = Vec::new();

    for (idx, line) in lines.into_iter().enumerate() {
        let words: Vec<&str> = line
            .split(" ")
            .collect();

        let has_4_words = words.len() == 4;

        if !has_4_words {
            println!("Error in config file (/use/local/etc/please.conf) at line {}", idx + 1);
            println!("Terminating");
        }

        let has_allow = words[0] == "allow";
        let has_as = words[2] == "as";
        let has_root = words[3] == "root";

        if has_allow && has_as && has_root {
            allowed_as_root.push(words[1]);
        }
    }

    if debug_mode {
        println!("Users permitted as root: {:?}", allowed_as_root);
    }

    let mut user_allowed = false;
    let current_user = env::var("USER");

    let current_user = match current_user {
        Ok(current_user) => current_user,
        Err(_) => {
            println!("Error reading $USER");
            process::exit(1);
        },
    };

    for user in allowed_as_root.into_iter() {
        if user == current_user {
            user_allowed = true;
        }
    }

    if !user_allowed {
        println!("User {} is not permitted to run as root.", current_user);
        process::exit(1);
    }

    // At this point, process should be running with root permissions
    // and current user is in the allowed list.

    let mut command_name = String::new();
    if (debug_mode && args.len() >= 3) || (args.len() >= 2) {
        if debug_mode {
            println!("Running command {}", args[2]);
            command_name = args[2].clone();
        } else {
            command_name = args[1].clone();
        }
    }

    let mut command_args: Vec<String> = Vec::new();
    if (debug_mode && args.len() > 3) || (args.len() > 2) {
        if debug_mode {
            command_args = args[3..].to_vec();
            println!("Args to command: {:?}", command_args);
        } else {
            command_args = args[2..].to_vec();
        }
    }

    let cmd = process::Command::new(command_name)
        .args(command_args)
        .spawn()
        .expect("Failed to spawn process");
}