use std::process;
use std::fs;

pub fn get_user_hash(user: String) -> String {
    let shadow_contents = fs::read_to_string("/etc/shadow");
    let shadow_contents = match shadow_contents {
        Ok(shadow_contents) => shadow_contents,
        Err(_) => {
            println!("Unable to read from /etc/shadow");
            process::exit(1);
        },
    };

    let shadow_contents: Vec<&str> = shadow_contents
        .split("\n")
        .filter(|&line| line != "")
        .collect();

    let mut user_shadow: String = "".to_string();

    for line in shadow_contents.into_iter() {
        let tokens: Vec<&str> = line
            .split(":")
            .collect();

        if tokens[0] == user {
            user_shadow = tokens.join(":");
            break;
        }
    }

    if user_shadow == "" {
        println!("User not found in /etc/shadow");
        process::exit(1);
    }

    let hash_string: &str = user_shadow
        .split(":")
        .collect::<Vec<&str>>()[1];

    hash_string.to_string()
}

pub fn get_salt(hash: String) -> String {
    let salt = hash.clone();
    let salt: Vec<&str> = salt
        .split("$")
        .collect();

    let mut salt_string: Vec<String> = Vec::new();
    salt_string.push("$".to_string());
    salt_string.push(salt[1].to_string());
    salt_string.push("$".to_string());
    salt_string.push(salt[2].to_string());
    let salt_string = salt_string.join("");

    salt_string
}

