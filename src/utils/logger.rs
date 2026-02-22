fn cool_log(log_data: &str) {
    println!("{log_data} 😎");
}

fn private_function() {}

#[cfg(test)]
mod tests {
    use super::*; // Выходим из папки tests в основной файл

    #[test]
    fn test_it() {
        private_function();
    }
}
