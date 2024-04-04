use crate::elements::PERIODIC_TABLE;

#[derive(Debug)]
pub enum InputError {
    InvalidCharacter,
    MultipleLowercaseAfterUppercase,
    UnknownElement(String),
}

pub fn extract_substrings(input: &str) -> Result<Vec<(String, usize)>, InputError> {
    let mut substrings = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(mandatory_upper) = chars.next() {
        let mut substring = String::new();
        if mandatory_upper.is_ascii_uppercase() {
            substring.push(mandatory_upper);
        } else {
            return Err(InputError::InvalidCharacter);
        }

        let mut lowercase_count = 0;
        let mut number = String::new();
        while let Some(next_char) = chars.peek() {
            if next_char.is_ascii_lowercase() {
                if lowercase_count == 1 {
                    return Err(InputError::MultipleLowercaseAfterUppercase);
                }
                substring.push(*next_char);
                lowercase_count += 1;
                chars.next();
            } else if next_char.is_ascii_digit() {
                number.push(*next_char);
                chars.next();
            } else {
                break;
            }
        }

        let num = if number.is_empty() {
            1
        } else {
            number
                .parse::<usize>()
                .map_err(|_| InputError::InvalidCharacter)?
        };

        if !PERIODIC_TABLE
            .iter()
            .any(|element| element.symbol == substring)
        {
            return Err(InputError::UnknownElement(substring));
        }

        substrings.push((substring, num));
    }

    if substrings.is_empty() {
        Err(InputError::InvalidCharacter)
    } else {
        Ok(substrings)
    }
}
