use std::process::Command;
use crate::{main, utils};

pub(crate) fn send(file: String, email_address: String) {
    let cmd = utils::execute_shell_cmd(format!("echo book | email -A {} {}", file, email_address));

    if cmd.1.len() > 0 {
        Command::new("notify-send")
            .arg(cmd.1.clone())
            .output()
            .expect("Failed to send a notification");

        println!("{}", cmd.1);

        main();
    } else {
        Command::new("notify-send")
            .arg(format!("File: {} sent successfully!", file))
            .output()
            .expect("Failed to send a notification");

        println!("File: {} sent successfully!", file);

        main();
    }
}

pub(crate) fn convert_and_send(file: String, email_address: String) {
    utils::execute_shell_cmd(format!("echo book | mail -s convert -a {} {}", file, email_address));
}
