pub fn log_action(action: &str) {
    let log_file = "cheat_log.txt";
    let mut file = std::fs::OpenOptions::new()
        .append(true)
        .create(true)
        .open(log_file)
        .expect("Unable to open log file");
    writeln!(file, "{}", action).expect("Unable to write to log file");
}

pub fn show_message(message: &str) {
    println!("{}", message);
}