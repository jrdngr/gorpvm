use crate::utils::parser::{Parser, literal, one_of, predicate, pair, take_while, optional, map};

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
    let number = |input| take_while(input, char::is_numeric);
    let postfix = |input| predicate(input, char::is_alphabetic);
    
    move |input| 
        map(pair(number, optional(postfix)), |(number, mode)| {
            let mode = match mode {
                Some(letter) => letter,
                None => String::from("i"),
            };
            
            (number, mode)
        })
        .parse(input)
}
