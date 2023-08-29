use nom::types::CompleteStr;
use nom::{named, do_parse, tag};
use crate::assembler::Token;
use crate::instruction::Opcode;

named!(
    pub opcode_load<CompleteStr, Token>,
    do_parse!(
        tag!("load") >> (Token::Op{code: Opcode::LOAD})
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_load_opcode() {
        let result = opcode_load(CompleteStr("load"));
        assert_eq!(result.is_ok(), true);
        let (rest, token) = result.unwrap();
        assert_eq!(token, Token::Op{ code: Opcode::LOAD });
        assert_eq!(rest, CompleteStr(""));

        let result = opcode_load(CompleteStr("aloa"));
        assert_eq!(result.is_ok(), false);
    }
}