use std::{fs, io};
use crate::{main, utils};
use serde::{Serialize, Deserialize};

/// Stores the configuration which would then be moved to /etc/ssmtp/ssmtp.conf
#[allow(non_snake_case)]
struct Configuration {
    root: String,
    mailhub: String,
    AuthUser: String,
    AuthPass: String,
    UseTLS: String,
    UseSTARTTLS: String,
}

/// Stores the User Credentials as provided by the User at runtime
struct Credentials {
    email: String,
    mailhub: String,
    password: String,
}

/// Stores the absolute location of the folder as referred to by the program
#[derive(Serialize, Deserialize)]
pub struct InternalConfig{
    pub(crate) folder: String,
    pub(crate) kindle_email_address: String
}

/// Takes an input from the user at runtime and returns the same as a string.
fn take_input() -> String {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read the email!");
    line.trim().to_string()
}

/// Checks if a configuration file is present in the directory.
pub(crate) fn is_config_present() -> bool {
    let ls = utils::execute_shell_cmd("ls".to_string());

    if ls.0.iter().any(|x| x == "config.json") {
        true
    } else {
        false
    }
}

/// Creates a configuration file and then appends it to the directory tree
pub(crate) fn update_config() {
    utils::execute_shell_cmd("sudo apt install mailutils ssmtp".to_string());

    /// Creates a configuration file for the structs Configuration and Credentials both together.
    fn config_for_creds() {

        let creds: Credentials = Credentials {
            email: {
                println!("Enter your email:");
                take_input()
            },
            mailhub: {
                println!("Enter the mailhub address");
                take_input()
            },
            password: {
                println!("Enter the password");
                rpassword::read_password().unwrap().to_string()
            },
        };

        let config: Configuration = Configuration {
            root: creds.email.clone(),
            AuthUser: creds.email.clone(),
            mailhub: creds.mailhub,
            AuthPass: creds.password,
            UseTLS: "YES".to_string(),
            UseSTARTTLS: "YES".to_string(),
        };

        let config = format!("root={}\n\
    AuthUser={}\n\
    mailhub={}\n\
    AuthPass={}\n\
    UseTLS={}\n\
    UseSTARTTLS={}",
                             config.root,
                             config.AuthUser,
                             config.mailhub,
                             config.AuthPass,
                             config.UseTLS,
                             config.UseSTARTTLS);
        fs::write("ssmtp.conf", config).expect("Failed to write the config to file");

        // moving the config file
        let cmd = utils::execute_shell_cmd("mv ssmtp.conf /etc/ssmtp/".to_string());
        println!("{}", cmd.0.join(" "));

        if cmd.1 != "" || cmd.1 != " " {
            println!("{}", cmd.1);
        }
    }

    /// Creates a configuration file for the folder (The folder for looking into for files)
    fn update_folder_config() {
        let config: InternalConfig = InternalConfig {
            folder: {
                println!("Enter the folder location (exact):");
                take_input()
            },
            kindle_email_address: {
                println!("Enter the Kindle's email address:");
                take_input()
            }
        };
        let json = serde_json::to_string(&config).expect("Failed to serialize config string to JSON");
        fs::write("config.json", json).expect("Failed to write the JSON to file");
    }

    config_for_creds();
    update_folder_config();
    main();
}