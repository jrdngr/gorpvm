mod parser;

use crate::parser::{Parser, literal, one_of, predicate, pair, one_or_more, optional, map};

pub fn opcode<'a>() -> impl Parser<'a, String> {
    move |input| {
        let parsers = vec![
            literal("hlt"),
            literal("ldr"),
            literal("str"),
            literal("set"),
            literal("cpy"),
            literal("jpt"),
            literal("jpf"),
            literal("add"),
            literal("sub"),
            literal("mul"),
            literal("div"),
            literal("mod"),
            literal("eql"),
            literal("neq"),
            literal("let"),
            literal("leq"),
            literal("grt"),
            literal("geq"),
            literal("sti"),
            literal("sto"),
        ];

        one_of(parsers).parse(input)
    }
}

pub fn value<'a>() -> impl Parser<'a, (String, String)> {
    move |input| {
        let number = |input| one_or_more(input, char::is_numeric);
        let postfix = |input| predicate(input, char::is_alphabetic);

        map(pair(number, optional(postfix)), |(number, mode)| {
            let mode = match mode {
                Some(letter) => letter,
                None => String::from("i"),
            };
            
            (number, mode)
        })
        .parse(input)
    }
}

pub fn parse_opcode(opcode: &str) -> u8 {
    match opcode {
        "hlt" => 0x00,
        "ldr" => 0x01,
        "str" => 0x02,
        "set" => 0x03,
        "cpy" => 0x04,
        "jpt" => 0x10,
        "jpf" => 0x11,
        "add" => 0x20,
        "sub" => 0x21,
        "mul" => 0x22,
        "div" => 0x23,
        "mod" => 0x24,
        "eql" => 0x30,
        "neq" => 0x31,
        "let" => 0x32,
        "leq" => 0x33,
        "grt" => 0x34,
        "geq" => 0x35,
        "sti" => 0x50,
        "sto" => 0x51,
        _     => panic!("Invalid opcode: {}", opcode),
    }
}

pub fn parse_value(value_number: &str, value_mode: &str) -> u8 {
    use std::str::FromStr;

    let number = u8::from_str(value_number).expect("Parsing error");
    let mode_bits = match value_mode {
        "i" => 0b1000_0000,
        "r" => 0b0001_0000,
        "o" => 0b0100_0000,
        _ => panic!("Invalid mode: {}", value_mode),
    };

    mode_bits | number
}

pub fn parse_instruction(instruction: &str) -> [u8; 4] {
    let (rest, opcode) = opcode().parse(instruction).expect("Parsing error");
    let opcode = parse_opcode(&opcode);
    if opcode == 0 {
        return [opcode, 0, 0, 0];
    }

    let (rest, _) = literal(" ").parse(rest).expect("Parsing error");
    let (rest, arg1) = value().parse(rest).expect("Parsing error");
    let (rest, _) = literal(" ").parse(rest).expect("Parsing error");
    let (rest, arg2) = value().parse(rest).expect("Parsing error");
    let (rest, _) = optional(literal(" ")).parse(rest).expect("Parsing error");
    let (_, arg3) = optional(value()).parse(rest).expect("Parsing error");

    let dest = parse_value(&arg1.0, &arg1.1);
    let op1 = parse_value(&arg2.0, &arg2.1);
    
    let op2 = match arg3 {
        Some(arg) => parse_value(&arg.0, &arg.1),
        None => 0,
    };

    [ opcode, dest, op1, op2 ]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_parser() {
        assert_eq!(opcode().parse("hlt"), Ok(("", String::from("hlt"))));
        assert_eq!(opcode().parse("taco"), Err("taco"));
    }

    #[test]
    fn opcode_parser() {
        assert_eq!(value().parse("123i"), Ok(("", (String::from("123"), String::from("i")))));
        assert_eq!(value().parse("123"), Ok(("", (String::from("123"), String::from("i")))));
        assert_eq!(value().parse("123b"), Ok(("", (String::from("123"), String::from("b")))));
        assert_eq!(value().parse("i"), Err("i"));
    }

    #[test]
    fn instruction_parser() {
        let i1 = parse_instruction("hlt");
        assert_eq!(i1, [0, 0, 0, 0]);

        let i2 = parse_instruction("ldr 0 1");
        assert_eq!(i2, [1, 128, 129, 0]);

        let i3 = parse_instruction("str 0 1");
        assert_eq!(i3, [2, 128, 129, 0]);
        
        let i4 = parse_instruction("add 0 1 2");
        assert_eq!(i4, [32, 128, 129, 130]);
    }
}
