fn gyro_show() -> Option<String> {
    let Ok(var) = std::env::var("GYRO_KEY") else { return None; };
    if var.is_empty() {
        return None;
    }
    panic!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn called_without_key() {
        std::env::remove_var("GYRO_KEY");
        let result = gyro_show();

        assert_eq!(result, None);
    }

    #[test]
    fn called_with_empty_key() {
        std::env::set_var("GYRO_KEY", "");
        let result = gyro_show();

        assert_eq!(result, None);
    }

    #[test]
    #[ignore]
    fn called_with_not_found_key() {
        std::env::set_var("GYRO_KEY", "BLAH");
        let result = gyro_show();

        assert_eq!(result, None);
    }
}
