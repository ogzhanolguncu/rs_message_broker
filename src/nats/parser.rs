use super::{
    commands::{handle_connect, handle_ping, handle_pub, handle_sub, handle_unsub, Command},
    errors::ErrMessages,
    parse_utils::{split_data_by_space, END_OF_LINE},
};

pub fn parse_nats(input: &str) -> Result<Command, ErrMessages> {
    let (head, tail) = split_data_by_space(input)?;
    match head.trim_end_matches(END_OF_LINE) {
        "SUB" => handle_sub(&tail),
        "PUB" => handle_pub(&tail),
        "UNSUB" => handle_unsub(&tail),
        "PING" => Ok(handle_ping()),
        "CONNECT" => Ok(handle_connect()),
        _ => Err(ErrMessages::UnknownCommand),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_sub_gracefully() {
        assert_eq!(
            Command::Sub {
                subject: "FOO".to_string(),
                sid: 1
            },
            parse_nats("SUB FOO 1\r\n").unwrap()
        )
    }

    #[test]
    fn shouldnt_parse_sub_due_to_unparseable_sid() {
        assert_eq!(
            ErrMessages::UnknownProtocalOperation,
            parse_nats("SUB FOO YOLO\r\n").unwrap_err()
        )
    }

    #[test]
    fn should_parse_pub_gracefully() {
        assert_eq!(
            Command::Pub {
                subject: "CodingChallenge".to_string(),
                payload: "Hello John!".to_string(),
            },
            parse_nats("PUB CodingChallenge 11\r\nHello John!\r\n").unwrap()
        )
    }

    #[test]
    fn should_not_parse_pub_due_to_malformed_payload() {
        assert_eq!(
            ErrMessages::UnknownProtocalOperation,
            parse_nats("PUB CodingChallenge 10\r\nHello John!\r\n").unwrap_err()
        )
    }

    #[test]
    fn should_not_parse_pub_due_to_unparseable_byte() {
        assert_eq!(
            ErrMessages::UnknownProtocalOperation,
            parse_nats("PUB CodingChallenge YOLO\r\nHello John!\r\n").unwrap_err()
        )
    }

    #[test]
    fn should_parse_ping() {
        assert_eq!(
            Command::Ping("PONG\r\n".to_string()),
            parse_nats("PING\r\n").unwrap()
        )
    }

    #[test]
    fn should_parse_connect() {
        assert_eq!(
            Command::Connect("+OK\r\n".to_string()),
            parse_nats("CONNECT {}\r\n").unwrap()
        )
    }
}
