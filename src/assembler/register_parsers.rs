use nom::types::CompleteStr;
use nom::{named, digit, ws, tag};
use crate::assembler::Token;

named!(
    pub register<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("$") >>
            register_number: digit >>
            (
                Token::Register {
                    reg_number: register_number.parse::<u8>().unwrap()
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_registers() {
        let result = register(CompleteStr("$0"));
        assert_eq!(result.is_ok(), true);
        let result = register(CompleteStr("$"));
        assert_eq!(result.is_ok(), false);
    }
}