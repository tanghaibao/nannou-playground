use crate::pest::Parser;

use nannou::math::deg_to_rad;
use pest::iterators::Pair;
use rand::{
    distributions::{Distribution, Standard},
    Rng,
};
use std::collections::HashMap;
use std::f32::consts::{FRAC_PI_2, FRAC_PI_3};

#[derive(Parser)]
#[grammar = "grammar/koch.pest"]
struct KochParser;

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum TurtleStep {
    Forward(char),
    ForwardNoLine,
    TurnLeft,
    TurnRight,
    Push,
    Pop,
    Reset,
    Node(char),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Koch {
    name: String,
    initial_state: Vec<TurtleStep>,
    rewrite_rules: HashMap<TurtleStep, Vec<TurtleStep>>,
    n: usize,
    state: Vec<TurtleStep>,
    delta: f32,
    step: usize, // current step in state
}

#[derive(Debug)]
pub enum KochModel {
    Cyclone,
    Caret,
    Islands,
    Xshape,
    Square,
    Grid,
    Sparse,
    Dense,
    Snowflake,
    Dragon,
    Sierpinski,
    HexGosper,
    QuadGosper,
    TreeA,
    TreeB,
    TreeC,
    TreeD,
    TreeE,
    TreeF,
}

impl Distribution<KochModel> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> KochModel {
        match rng.gen_range(0..=18) {
            0 => KochModel::Cyclone,
            1 => KochModel::Caret,
            2 => KochModel::Islands,
            3 => KochModel::Xshape,
            4 => KochModel::Square,
            5 => KochModel::Grid,
            6 => KochModel::Sparse,
            7 => KochModel::Dense,
            8 => KochModel::Snowflake,
            9 => KochModel::Dragon,
            10 => KochModel::Sierpinski,
            11 => KochModel::HexGosper,
            12 => KochModel::QuadGosper,
            13 => KochModel::TreeA,
            14 => KochModel::TreeB,
            15 => KochModel::TreeC,
            16 => KochModel::TreeD,
            17 => KochModel::TreeE,
            18 => KochModel::TreeF,
            _ => unreachable!(),
        }
    }
}

fn parse_step(r: Pair<Rule>) -> TurtleStep {
    match r.as_rule() {
        Rule::forward => TurtleStep::Forward(r.as_str().chars().next().unwrap()),
        Rule::forward_no_line => TurtleStep::ForwardNoLine,
        Rule::turn_left => TurtleStep::TurnLeft,
        Rule::turn_right => TurtleStep::TurnRight,
        Rule::push => TurtleStep::Push,
        Rule::pop => TurtleStep::Pop,
        Rule::node => TurtleStep::Node(r.as_str().chars().next().unwrap()),
        _ => panic!("unexpected rule: {:?}", r.as_rule()),
    }
}

fn parse_steps(s: &str) -> Vec<TurtleStep> {
    let state = KochParser::parse(Rule::state, s)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap();

    state.into_inner().map(parse_step).collect()
}

fn parse_rewrite_rules(s: &str) -> HashMap<TurtleStep, Vec<TurtleStep>> {
    let productions = KochParser::parse(Rule::productions, s)
        .expect("unsuccessful parse") // unwrap the parse result
        .next()
        .unwrap()
        .into_inner();

    let mut rules = HashMap::new();
    let mut from = TurtleStep::Forward('F');
    for r in productions {
        match r.as_rule() {
            Rule::production => {
                for ir in r.into_inner() {
                    match ir.as_rule() {
                        Rule::predecessor => from = parse_step(ir.into_inner().next().unwrap()),
                        Rule::state => {
                            let to = ir.into_inner().map(parse_step).collect();
                            rules.insert(from.clone(), to);
                        }
                        _ => panic!("unexpected rule: {:?}", ir.as_rule()),
                    }
                }
            }
            Rule::EOI => (),
            _ => panic!("unexpected rule: {:?}", r.as_rule()),
        }
    }
    rules
}

impl Koch {
    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_delta(&self) -> f32 {
        self.delta
    }

    pub fn model(m: KochModel) -> Koch {
        match m {
            KochModel::Cyclone => Self::cyclone(),
            KochModel::Caret => Self::caret(),
            KochModel::Islands => Self::islands(),
            KochModel::Xshape => Self::xshape(),
            KochModel::Square => Self::square(),
            KochModel::Grid => Self::grid(),
            KochModel::Sparse => Self::sparse(),
            KochModel::Dense => Self::dense(),
            KochModel::Snowflake => Self::snowflake(),
            KochModel::Dragon => Self::dragon(),
            KochModel::Sierpinski => Self::sierpinski(),
            KochModel::HexGosper => Self::hex_gosper(),
            KochModel::QuadGosper => Self::quad_gosper(),
            KochModel::TreeA => Self::tree_a(),
            KochModel::TreeB => Self::tree_b(),
            KochModel::TreeC => Self::tree_c(),
            KochModel::TreeD => Self::tree_d(),
            KochModel::TreeE => Self::tree_e(),
            KochModel::TreeF => Self::tree_f(),
        }
    }

    fn cyclone() -> Self {
        Self::from("cyclone", "F-F-F-F", "F => F-F+F+FF-F-F+F", 2, FRAC_PI_2)
    }

    fn caret() -> Self {
        Self::from("caret", "-F", "F => F+F-F-F+F", 4, FRAC_PI_2)
    }

    fn islands() -> Self {
        Self::from(
            "islands",
            "F+F+F+F",
            "F => F+f-FF+F+FF+Ff+FF-f+FF-F-FF-Ff-FFF\nf => ffffff",
            2,
            FRAC_PI_2,
        )
    }

    fn xshape() -> Self {
        Self::from("xshape", "F-F-F-F", "F => FF-F-F-F-F-F+F", 4, FRAC_PI_2)
    }

    fn square() -> Self {
        Self::from("square", "F-F-F-F", "F => FF-F-F-F-FF", 4, FRAC_PI_2)
    }

    fn grid() -> Self {
        Self::from("grid", "F-F-F-F", "F => FF-F+F-F-FF", 3, FRAC_PI_2)
    }

    fn sparse() -> Self {
        Self::from("sparse", "F-F-F-F", "F => FF-F--F-F", 4, FRAC_PI_2)
    }

    fn dense() -> Self {
        Self::from("dense", "F-F-F-F", "F => F-FF--F-F", 5, FRAC_PI_2)
    }

    fn snowflake() -> Self {
        Self::from("snowflake", "F-F-F-F", "F => F-F+F-F-F", 4, FRAC_PI_2)
    }

    fn dragon() -> Self {
        Self::from("dragon", "L", "L => L+R+\nR => -L-R", 10, FRAC_PI_2)
    }

    fn sierpinski() -> Self {
        Self::from("sierpinski", "R", "L => R+L+R\nR => L-R-L", 6, FRAC_PI_3)
    }

    fn hex_gosper() -> Self {
        Self::from(
            "hex_gosper",
            "L",
            "L => L+R++R-L--LL-R+\nR => -L+RR++R+L--L-R",
            4,
            FRAC_PI_3,
        )
    }

    fn quad_gosper() -> Self {
        Self::from("quad_gosper","-R", "L => LL-R-R+L+L-R-RL+R+LLR-L+R+LL+R-LR-R-L+L+RR-\nR => +LL-R-R+L+LR+L-RR-L-R+LRR-L-RL+L+R-R-L+L+RR", 2, FRAC_PI_2)
    }

    fn tree_a() -> Self {
        Self::from("tree_a", "F", "F => F[+F]F[-F]F", 5, deg_to_rad(25.7))
    }

    fn tree_b() -> Self {
        Self::from("tree_b", "F", "F => F[+F]F[-F][F]", 5, deg_to_rad(20.0))
    }

    fn tree_c() -> Self {
        Self::from(
            "tree_c",
            "F",
            "F => FF-[-F+F+F]+[+F-F-F]",
            5,
            deg_to_rad(22.5),
        )
    }

    fn tree_d() -> Self {
        Self::from(
            "tree_d",
            "X",
            "X => F[+X]F[-X]+X\nF => FF",
            7,
            deg_to_rad(20.0),
        )
    }

    fn tree_e() -> Self {
        Self::from(
            "tree_e",
            "X",
            "X => F[+X][-X]FX\nF => FF",
            7,
            deg_to_rad(25.7),
        )
    }

    fn tree_f() -> Self {
        Self::from(
            "tree_f",
            "X",
            "X => F-[[X]+X]+F[+FX]-X\nF => FF",
            5,
            deg_to_rad(22.5),
        )
    }

    fn from(name: &str, initial_state: &str, rewrite_rules: &str, n: usize, delta: f32) -> Self {
        let initial_state = parse_steps(initial_state);
        let rewrite_rules = parse_rewrite_rules(rewrite_rules);
        let state = initial_state.clone();
        let mut koch = Koch {
            name: name.to_string(),
            initial_state,
            rewrite_rules,
            state,
            n,
            delta,
            step: 0,
        };
        for _ in 0..n {
            koch.next_iteration();
        }
        koch
    }

    pub fn next_step(&mut self) -> TurtleStep {
        if self.step >= self.state.len() {
            self.step = 0;
            return TurtleStep::Reset;
        }
        let s = self.state[self.step].clone();
        self.step += 1;
        s
    }

    pub fn next_iteration(&mut self) {
        let mut new_state = Vec::new();
        for step in &self.state {
            if let Some(rule) = self.rewrite_rules.get(&step) {
                new_state.extend(rule.iter().cloned());
            } else {
                new_state.push(step.clone());
            }
        }
        self.state = new_state;
        self.step = 0;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constructor() {
        let koch = super::Koch::from("test", "F", "F => F+F-", 0, FRAC_PI_2);
        assert_eq!(koch.initial_state, vec![TurtleStep::Forward('F')]);
        assert_eq!(
            koch.rewrite_rules,
            [(
                TurtleStep::Forward('F'),
                vec![
                    TurtleStep::Forward('F'),
                    TurtleStep::TurnLeft,
                    TurtleStep::Forward('F'),
                    TurtleStep::TurnRight,
                ]
            )]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn dragon() {
        let dragon = Koch::dragon();
        assert_eq!(dragon.initial_state, vec![TurtleStep::Forward('L')]);
    }

    #[test]
    fn quad_gosper() {
        let quad_gosper = Koch::quad_gosper();
        assert_eq!(
            quad_gosper.initial_state,
            vec![TurtleStep::TurnRight, TurtleStep::Forward('R')]
        );
    }

    #[test]
    fn step() {
        let mut koch = super::Koch::from("test", "F", "F => F+F-", 0, FRAC_PI_2);
        koch.next_iteration();
        assert_eq!(
            koch.state,
            vec![
                TurtleStep::Forward('F'),
                TurtleStep::TurnLeft,
                TurtleStep::Forward('F'),
                TurtleStep::TurnRight
            ]
        );
        koch.next_iteration();
        assert_eq!(
            koch.state,
            vec![
                TurtleStep::Forward('F'),
                TurtleStep::TurnLeft,
                TurtleStep::Forward('F'),
                TurtleStep::TurnRight,
                TurtleStep::TurnLeft,
                TurtleStep::Forward('F'),
                TurtleStep::TurnLeft,
                TurtleStep::Forward('F'),
                TurtleStep::TurnRight,
                TurtleStep::TurnRight,
            ]
        );
    }
}
