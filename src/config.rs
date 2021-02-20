use std::fs;
use std::process;
use std::env;

pub fn read_config() -> Vec<String> {
    let config_contents = fs::read_to_string("/usr/local/etc/please.conf");

    let config_contents = match config_contents {
        Ok(config_contents) => config_contents,
        Err(_) => {
            println!("Error opening config file at /usr/local/etc/please.conf");
            process::exit(1);
        },
    };

    let lines: Vec<&str> = config_contents
        .split("\n")
        .filter(|&line| line != "" && line != " ")
        .collect();

    let mut allowed_as_root: Vec<String> = Vec::new();

    for (idx, line) in lines.into_iter().enumerate() {
        let words: Vec<&str> = line
            .split(" ")
            .collect();

        let has_4_words = words.len() == 4;

        if !has_4_words {
        println!("Error in config file /usr/local/etc/please.conf at line {}", idx + 1);
        process::exit(1);
        }

        let mut is_valid = words[0] == "allow".to_string();
        is_valid &= words[2] == "as".to_string();
        is_valid &= words[3] == "root".to_string();

        if is_valid {
            allowed_as_root.push(words[1].to_string());
        }
    }

    allowed_as_root
}

pub fn get_user() -> String {
    let current_user = env::var("USER");

    let current_user = match current_user {
        Ok(current_user) => current_user,
        Err(_) => {
            println!("Error reading $USER");
            process::exit(1);
        }
    };

    current_user.to_string()
}

pub fn is_allowed(current_user: &String, allowed: Vec<String>) -> bool {
    for user in allowed.into_iter() {
        if user == *current_user {
            return true;
        }
    }
    return false;
}
