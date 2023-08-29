use nom::types::CompleteStr;
use nom::{named, digit, ws, tag};
use crate::assembler::Token;

named!(
    pub integer_operand<CompleteStr, Token>,
    ws!(
        do_parse!(
            tag!("#") >>
            number: digit >>
            (
                Token::IntegerOperand {
                    value: number.parse::<i32>().unwrap()
                }
            )
        )
    )
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_integer_operand() {
        let result = integer_operand(CompleteStr("#0"));
        assert_eq!(result.is_ok(), true);
        let (rest, value) = result.unwrap();
        assert_eq!(value, Token::IntegerOperand { value: 0 });
        assert_eq!(rest, CompleteStr(""));

        let result = integer_operand(CompleteStr("0"));
        assert_eq!(result.is_ok(), false);
    }
}