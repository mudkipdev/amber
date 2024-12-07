pub const SPACE: char = ' ';

pub fn take_while(accept: impl Fn(char) -> bool, string: &str) -> (&str, &str) {
    let end = string
        .char_indices()
        .find(|(_, character)| !accept(*character))
        .map(|(index, _)| index)
        .unwrap_or(string.len());

    let extracted = &string[..end];
    let remainder = &string[end..];
    (remainder, extracted)
}

pub fn extract_whitespace(string: &str) -> (&str, &str) {
    take_while(|character| character == SPACE, string)
}

pub fn extract_digits(string: &str) -> (&str, &str) {
    take_while(|character| character.is_ascii_digit(), string)
}

pub fn extract_operator(string: &str) -> Result<(&str, &str), ()> {
    match &string[0..1] {
        "+" | "-" | "*" | "/" => (),
        _ => return Err(()),
    }

    Ok((&string[1..], &string[0..1]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn extract_spaces() {
        assert_eq!(extract_whitespace("    1"), ("1", "    "));
    }

    #[test]
    fn extract_single_digit() {
        assert_eq!(extract_digits("1+2"), ("+2", "1"));
    }

    #[test]
    fn extract_multiple_digits() {
        assert_eq!(extract_digits("10-20"), ("-20", "10"));
    }

    #[test]
    fn no_extraction_from_empty_input() {
        assert_eq!(extract_digits(""), ("", ""));
    }

    #[test]
    fn extract_digits_with_no_remainder() {
        assert_eq!(extract_digits("100"), ("", "100"));
    }

    #[test]
    fn extract_addition_operator() {
        assert_eq!(extract_operator("+2").unwrap(), ("2", "+"));
    }

    #[test]
    fn extract_subtraction_operator() {
        assert_eq!(extract_operator("-10").unwrap(), ("10", "-"));
    }

    #[test]
    fn extract_multiplication_operator() {
        assert_eq!(extract_operator("*3").unwrap(), ("3", "*"));
    }

    #[test]
    fn extract_division_operator() {
        assert_eq!(extract_operator("/4").unwrap(), ("4", "/"));
    }
}
