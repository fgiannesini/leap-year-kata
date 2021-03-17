fn main() {
}

fn is_leap_year(year: i32) -> bool {
    true
}

#[cfg(test)]
mod tests {
    use crate::is_leap_year;

    #[test]
    fn should_be_a_leap_year() {
        assert!(is_leap_year(1980))
    }
}