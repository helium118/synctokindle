use std::process::Command;

/// Returns a tuple.
/// The first element is a Vector of the output and the Second element is the error as a String
///
/// # Arguments
///
/// * `cmd` - The command to be executed
///
pub(crate) fn execute_shell_cmd(cmd: String) -> (Vec<String>, String) {
    let mut cmd_split = cmd.split(" ").collect::<Vec<&str>>();
    let cmd = cmd_split[0];
    cmd_split.remove(0);
    let args = cmd_split;

    let output = Command::new(cmd)
        .args(args)
        .output()
        .expect("Failed to execute the command!");

    let error = match String::from_utf8(output.stderr) {
        Ok(str) => str,
        Err(_) => panic!("Failed to parse error from utf encoded string"),
    };

    let output = match String::from_utf8(output.stdout) {
        Ok(str) => str::replace(&str[..], "\n", " ")
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>(),

        Err(_) => panic!("Failed to parse output from utf encoded string"),
    };
    (output, error)
}

pub(crate) fn monitor_dir_for_changes(directory: String) -> String {
    let cmd = execute_shell_cmd(format!("inotifywait -e move -e create {}", directory));

    let file = directory + "/" + &cmd.0[2];

    file.clone()
}

pub(crate) fn is_compatible(file: String) -> bool {
    let supported_file_types = [
        ".doc", ".docx", ".pdf", ".txt", ".jpg", ".jpeg", ".png", ".azw", ".mobi", ".rtf", ".prc",
        ".psz", ".bmp",
    ];

    let file_type = format!(
        "{}{}",
        ".".to_string(),
        file.split(".")
            .map(|x| x.to_string())
            .collect::<Vec<String>>()[1]
            .clone()
    );

    if supported_file_types.contains(&&file_type[..]) {
        true
    } else {
        false
    }
}
