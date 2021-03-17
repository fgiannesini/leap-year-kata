fn main() {
}

fn is_leap_year(year: i32) -> bool {
    year %4 == 0 && year != 1900
}

#[cfg(test)]
mod tests {
    use crate::is_leap_year;

    #[test]
    fn should_be_a_leap_year_if_multiple_of_4() {
        assert!(is_leap_year(1980))
    }

    #[test]
    fn should_be_an_other_leap_year_if_multiple_of_4() {
        assert!(is_leap_year(1984))
    }

    #[test]
    fn should_not_be_a_leap_year_if_multiple_of_100() {
        assert!(!is_leap_year(1900))
    }

    #[test]
    fn should_not_be_a_leap_year() {
        assert!(!is_leap_year(1981))
    }
}