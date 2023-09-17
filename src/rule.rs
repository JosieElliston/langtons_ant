// #[derive(Clone, Copy)]
// pub(crate) struct Rule(i8);

pub(crate) type Rule = i32;
pub(crate) const RULE_L: Rule = 1;
pub(crate) const RULE_C: Rule = 0;
pub(crate) const RULE_R: Rule = -1;

pub(crate) fn rule_vec_from_str(s: &str) -> Vec<Rule> {
    s.chars()
        .map(|c| match c {
            'L' => RULE_L,
            'C' => RULE_C,
            'R' => RULE_R,
            _ => panic!("invalid char in string"),
        })
        .collect()
}

// impl Rule {
//     // pub(crate) const LEFT3: Rule = Rule(3);
//     // pub(crate) const L2: Rule = Rule(2);
//     pub(crate) const L: Rule = Rule(1);
//     pub(crate) const C: Rule = Rule(0);
//     pub(crate) const R: Rule = Rule(-1);
//     // pub(crate) const R2: Rule = Rule(-2);

//     pub(crate) fn get(&self) -> i8 {
//         self.0
//     }

//     pub(crate) fn from_str(s: &str) -> Vec<Rule> {
//         s.chars().map(
//             |c| {
//                 match c {
//                     'L' => Rule::L,
//                     'C' => Rule::C,
//                     'R' => Rule::R,
//                     _ => panic!("invalid char in string")
//                 }
//             }
//         ).collect()
//     }
// }

// impl std::fmt::Display for Rule {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(
//             f,
//             "{}",
//             match self.0 {
//                 2 => "left 2",
//                 1 => "left",
//                 0 => "center",
//                 -1 => "right",
//                 -2 => "right 2",
//                 _ => panic!("invalid rule"),
//             }
//         )
//     }
// }
