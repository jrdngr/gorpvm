pub enum Instruction {
    Load(usize, usize),
    Store(usize, usize),
    Add,
    Sub,
    Mul,
    Div,
    Mod,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
