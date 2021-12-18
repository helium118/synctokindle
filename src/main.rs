use std::{fs, process, string};

mod mail;
mod pre_setup;
mod utils;

fn main() {
    if !pre_setup::is_config_present() {
        pre_setup::update_config(); // should call main again
    } else {
        // Checking if the config directory is correct
        let config: pre_setup::InternalConfig = serde_json::from_str(
            &string::String::from_utf8(fs::read("config.json").unwrap()).unwrap()[..],
        )
        .unwrap();

        if utils::execute_shell_cmd(format!("dir {}", config.folder))
            .1
            .len()
            > 0
        {
            println!("The directory doesn't exist.\nRerun with the --update flag and enter a valid directory.");
            process::exit(1);
        } else {
            let file = utils::monitor_dir_for_changes(config.folder);

            if file.clone().to_lowercase().contains("convert") && utils::is_compatible(file.clone())
            {
                mail::convert_and_send(file.clone(), config.kindle_email_address)
            } else if utils::is_compatible(file.clone()) {
                mail::send(file.clone(), config.kindle_email_address)
            } else {
                println!(
                    "The file format is not supported.\n\nRefer to this site \
                (https://www.amazon.com/gp/help/customer/display.html?nodeId=G5WYD9SAF7PGXRNA)\
                for a list of supported file formats"
                )
            }
        }
    }
}
