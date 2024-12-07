#[derive(PartialEq, Debug)]
pub struct Equation {
    pub result: usize,
    pub values: Vec<usize>,
}

#[derive(Debug)]
pub enum Operator {
    Mul,
    Add,
    Combine,
}

fn number_of_digits(n: usize) -> u32 {
    if n == 0 {
        return 1;
    }
    n.ilog10() + 1
}

impl Operator {
    pub fn apply(&self, lhs: usize, rhs: usize) -> usize {
        match self {
            Operator::Mul => lhs * rhs,
            Operator::Add => lhs + rhs,
            Operator::Combine => lhs * 10_usize.pow(number_of_digits(rhs)) + rhs,
        }
    }
}

pub const OPERATORS_PART1: &[Operator] = &[Operator::Mul, Operator::Add];
pub const OPERATORS_PART2: &[Operator] = &[Operator::Mul, Operator::Add, Operator::Combine];

pub fn read_equations(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(|l| {
            let mut tokens = l.split_whitespace();
            Equation {
                result: tokens.next().unwrap().replace(":", "").parse().unwrap(),
                values: tokens.map(|t| t.parse().unwrap()).collect(),
            }
        })
        .collect()
}

pub fn evaluate(values: &[usize], operators: &[&Operator]) -> usize {
    let mut lhs = values[0];
    for (value_index, operator) in operators.iter().enumerate() {
        lhs = operator.apply(lhs, values[value_index + 1]);
    }
    lhs
}

pub fn equation_is_true(equation: &Equation, operators: &[Operator]) -> bool {
    use itertools::Itertools;
    for operators in std::iter::repeat(operators)
        .take(equation.values.len() - 1)
        .multi_cartesian_product()
    {
        if evaluate(&equation.values, &operators) == equation.result {
            return true;
        }
    }
    false
}

pub fn total_calibration_result(equations: &[Equation], operators: &[Operator]) -> usize {
    equations
        .iter()
        .filter(|e| equation_is_true(e, operators))
        .map(|e| e.result)
        .sum()
}

#[test]
fn test_number_of_digits() {
    assert_eq!(number_of_digits(0), 1);
    assert_eq!(number_of_digits(1), 1);
    assert_eq!(number_of_digits(99), 2);
    assert_eq!(number_of_digits(100), 3);
    assert_eq!(number_of_digits(9999), 4);
}

#[test]
fn test_equation_is_true() {
    assert!(equation_is_true(
        &Equation {
            result: 3267,
            values: vec![81, 40, 27]
        },
        OPERATORS_PART1,
    ));
    assert!(!equation_is_true(
        &Equation {
            result: 7290,
            values: vec![6, 8, 6, 15]
        },
        OPERATORS_PART1,
    ));
    assert!(equation_is_true(
        &Equation {
            result: 7290,
            values: vec![6, 8, 6, 15]
        },
        OPERATORS_PART2,
    ));
}

#[test]
fn day() {
    let data1 = r#"
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#
        .trim();

    let equations1 = read_equations(data1);

    assert_eq!(
        equations1[0],
        Equation {
            result: 190,
            values: vec![10, 19]
        }
    );
    assert_eq!(total_calibration_result(&equations1, OPERATORS_PART1), 3749);

    let data2 = std::fs::read_to_string("input/day7").unwrap();
    let equations2 = read_equations(&data2);
    assert_eq!(
        total_calibration_result(&equations2, OPERATORS_PART1),
        5837374519342
    );
    assert_eq!(
        total_calibration_result(&equations2, OPERATORS_PART2),
        492383931650959
    );
}
