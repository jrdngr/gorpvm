use crate::cpu::instruction::Instruction;
use crate::utils::parser::{Parser, literal, one_of, predicate, pair, one_or_more, optional, map};

pub fn opcode<'a>() -> impl Parser<'a, String> {
    move |input| {
        let parsers = vec![
            literal("halt"),
            literal("load"),
            literal("store"),
            literal("set"),
            literal("copy"),
            literal("jmpt"),
            literal("jmpf"),
            literal("add"),
            literal("sub"),
            literal("mul"),
            literal("div"),
            literal("mod"),
            literal("eq"),
            literal("ne"),
            literal("lt"),
            literal("le"),
            literal("gt"),
            literal("ge"),
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
        "halt"  => 0x00,
        "load"  => 0x01,
        "store" => 0x02,
        "set"   => 0x03,
        "copy"  => 0x04,
        "jmpt"  => 0x10,
        "jmpf"  => 0x11,
        "add"   => 0x20,
        "sub"   => 0x21,
        "mul"   => 0x22,
        "div"   => 0x23,
        "mod"   => 0x24,
        "eq"    => 0x30,
        "ne"    => 0x31,
        "lt"    => 0x32,
        "le"    => 0x33,
        "gt"    => 0x34,
        "ge"    => 0x35,
        _       => panic!("Invalid opcode: {}", opcode),
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

pub fn parse_instruction(instruction: &str) -> Instruction {
    let (rest, opcode) = opcode().parse(instruction).expect("Parsing error");
    let _ = literal(" ").parse(rest);
    let (rest, arg1) = value().parse(rest).expect("Parsing error");
    let _ = literal(" ").parse(rest);
    let (rest, arg2) = value().parse(rest).expect("Parsing error");
    let _ = optional(literal(" ")).parse(rest);
    let (_, arg3) = optional(value()).parse(rest).expect("Parsing error");

    let opcode = parse_opcode(&opcode);
    if opcode == 0 {
        return Instruction { opcode, src1: 0, src2: 0, dest: 0 };
    }

    let src1 = parse_value(&arg1.0, &arg1.1);

    let mut src2 = parse_value(&arg2.0, &arg2.1);
    let dest = match arg3 {
        Some(arg) => parse_value(&arg.0, &arg.1),
        None => { 
            let result = src2;
            src2 = 0;
            result
        },
    };

    Instruction {
        opcode,
        src1,
        src2,
        dest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_parser() {
        assert_eq!(opcode().parse("halt"), Ok(("", String::from("halt"))));
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
        let i1 = parse_instruction("halt");
        assert_eq!(i1, Instruction {
            opcode: 0,
            src1: 0,
            src2: 0,
            dest: 0,
        });

        let i2 = parse_instruction("load 0 1");
        
        
        let i3 = parse_instruction("store 0 1");
        
        
        let i4 = parse_instruction("add 0 1 2");
    }
}
