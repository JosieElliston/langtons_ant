#[derive(Clone, Copy)]
pub(crate) struct Rule(i8);

impl Rule {
    // pub(crate) const LEFT3: Rule = Rule(3);
    pub(crate) const L2: Rule = Rule(2);
    pub(crate) const L: Rule = Rule(1);
    pub(crate) const C: Rule = Rule(0);
    pub(crate) const R: Rule = Rule(-1);
    pub(crate) const R2: Rule = Rule(-2);

    pub(crate) fn get(&self) -> i8 {
        self.0
    }
}

impl std::fmt::Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                2 => "left 2",
                1 => "left",
                0 => "center",
                -1 => "right",
                -2 => "right 2",
                _ => panic!("invalid rule"),
            }
        )
    }
}