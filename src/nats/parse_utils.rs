use std::str::Split;

use super::errors::ErrMessages;

pub static SPACE_SEPERATOR: &str = " ";
pub static END_OF_LINE: &str = "\r\n";

pub fn split_data_by_space(input: &str) -> Result<(String, String), ErrMessages> {
    let mut head_and_tail: Split<'_, &str> = input.split(SPACE_SEPERATOR);
    let head = head_and_tail
        .next()
        .ok_or(ErrMessages::NoHeadElement)?
        .to_string();
    let tail: String = head_and_tail
        .collect::<Vec<&str>>()
        .join(SPACE_SEPERATOR)
        .to_string();
    Ok((head, tail))
}

pub fn split_data_by_crlf(input: &str) -> Result<(String, String), ErrMessages> {
    let mut head_and_tail: Split<'_, &str> = input.split(END_OF_LINE);
    let head = head_and_tail
        .next()
        .ok_or(ErrMessages::NoHeadElement)?
        .to_string();
    let tail: String = head_and_tail
        .collect::<Vec<&str>>()
        .join(SPACE_SEPERATOR)
        .trim_end()
        .to_string();
    Ok((head, tail))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_split_ping_correctly() {
        assert_eq!(
            ("PING\r\n".to_string(), "".to_string()),
            split_data_by_space("PING\r\n").unwrap()
        );
    }

    #[test]
    fn should_split_correct_correctly() {
        assert_eq!(
            ("CONNECT".to_string(), "{} \r\n".to_string()),
            split_data_by_space("CONNECT {} \r\n").unwrap()
        );
    }

    #[test]
    fn should_split_sub_correctly() {
        assert_eq!(
            ("SUB".to_string(), "FOO 1\r\n".to_string()),
            split_data_by_space("SUB FOO 1\r\n").unwrap()
        );
    }

    #[test]
    fn should_split_pub_correctly() {
        assert_eq!(
            (
                "PUB".to_string(),
                "CodingChallenge 11\r\nHello John!\r\n".to_string()
            ),
            split_data_by_space("PUB CodingChallenge 11\r\nHello John!\r\n").unwrap()
        );
    }

    #[test]
    fn should_split_data_by_crlf() {
        assert_eq!(
            (11.to_string(), "Hello John!".to_string()),
            split_data_by_crlf("11\r\nHello John!\r\n").unwrap()
        );
    }
}
