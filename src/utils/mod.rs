pub mod logger;

pub fn make_log(log_data: &str) {
    // super::logger::cool_log(log_data);
    println!("Log data: {}", format!("{} suka!", log_data));
}
