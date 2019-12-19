// Following https://bodil.lol/parser-combinators/

pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
}

impl<'a, F, Output> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParseResult<Output>,
{
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

pub fn literal<'a>(expected: &'static str) -> impl Parser<'a, String> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], expected.to_string())),
        _ => Err(input)
    }
}

pub fn predicate<F>(input: &str, predicate: F) -> ParseResult<String> 
where
    F: Fn(char) -> bool,
{
    match input.chars().nth(0) {
        Some(ch) if predicate(ch) => Ok((&input[1..], ch.to_string())),
        _ => Err(input),
    }
}

pub fn take_while<F>(input: &str, predicate: F) -> ParseResult<String> 
where
    F: Fn(char) -> bool,
{
    let mut matched = String::new();
    let mut chars = input.chars();

    while let Some(next) = chars.next() {
        if predicate(next) {
            matched.push(next);
        } else {
            break;
        }
    }

    let next_index = matched.len();
    Ok((&input[next_index..], matched))
}

pub fn pair<'a, P1, P2, R1, R2>(parser1: P1, parser2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| {
        parser1.parse(input)
            .and_then(|(next_input, result1)| {
                parser2.parse(next_input)
                    .map(|(last_input, result2)| (last_input, (result1, result2))) 
            })
    }
}

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_input, result)| (next_input, map_fn(result)))
    }
}

pub fn one_of<'a, P, R>(parsers: Vec<P>) -> impl Parser<'a, R> 
where
    P: Parser<'a, R>
{
    move |input| {
        let mut parsers = parsers.iter();

        while let Some(parser) = parsers.next() {
            if let Ok(result) = parser.parse(input) {
                return Ok(result);
            }
        }

        Err(input)
    }
}

pub fn optional<'a, P, R>(parser: P) -> impl Parser<'a, Option<R>>
where
    P: Parser<'a, R>,
{
    move |input| {
        match parser.parse(input) {
            Ok((rest, result)) => Ok((rest, Some(result))),
            Err(input) => Ok((input, None)),
        }
    }
}
