#![allow(dead_code)]
#![allow(clippy::while_let_on_iterator)]

use anyhow::Result;
use text_io::try_scan;

#[derive(Debug)]
struct Rule {
    before: usize,
    after: usize,
}

#[derive(Debug, Clone)]
struct Update {
    pages: Vec<usize>,
}

#[derive(Debug)]
struct Instructions {
    rules: Vec<Rule>,
    updates: Vec<Update>,
}

fn read_instructions(input: &str) -> Result<Instructions> {
    let mut lines = input.lines();
    let mut rules = Vec::new();
    for line in &mut lines {
        if line.trim().is_empty() {
            break;
        }
        let (before, after): (usize, usize);
        try_scan!(line.bytes() => "{}|{}", before, after);
        rules.push(Rule { before, after });
    }
    let updates: Vec<Update> = lines
        .map(|line| Update {
            pages: line.split(',').map(|n| n.parse().unwrap()).collect(),
        })
        .collect();

    Ok(Instructions { rules, updates })
}

fn is_correctly_ordered(update: &Update, rules: &[Rule]) -> bool {
    rules.iter().all(|rule| {
        let index_before = update.pages.iter().position(|&p| p == rule.before);
        let index_after = update.pages.iter().position(|&p| p == rule.after);
        match (index_before, index_after) {
            (Some(before), Some(after)) => before < after,
            _ => true,
        }
    })
}

fn fix_update(mut update: Update, rules: &Vec<Rule>) -> Update {
    let mut changed = true;
    while changed {
        changed = false;
        for rule in rules {
            if let (Some(index_before), Some(index_after)) = (
                update.pages.iter().position(|&p| p == rule.before),
                update.pages.iter().position(|&p| p == rule.after),
            ) {
                if index_before > index_after {
                    if index_before == update.pages.len() - 1 {
                        update.pages.remove(index_after);
                        update.pages.push(rule.after);
                    } else {
                        update.pages.remove(index_after);
                        update.pages.insert(index_before + 1, rule.after);
                    }
                    changed = true;
                }
            };
        }
    }
    update
}

fn sum_correctly_ordered(instructions: &Instructions) -> usize {
    instructions
        .updates
        .iter()
        .filter(|u| is_correctly_ordered(u, &instructions.rules))
        .map(|u| u.pages[u.pages.len() / 2])
        .sum()
}

fn sum_incorrectly_ordered(instructions: &Instructions) -> Result<usize> {
    Ok(instructions
        .updates
        .iter()
        .filter(|u| !is_correctly_ordered(u, &instructions.rules))
        .cloned()
        .map(|u| fix_update(u, &instructions.rules))
        .map(|u| u.pages[u.pages.len() / 2])
        .sum())
}

#[test]
fn day() -> Result<()> {
    let data1 = r#"
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
"#
    .trim();
    let instructions1 = read_instructions(data1)?;
    assert_eq!(instructions1.rules.len(), 21);
    assert_eq!(instructions1.updates.len(), 6);
    assert_eq!(sum_correctly_ordered(&instructions1), 143);
    let data2 = std::fs::read_to_string("input/day5").unwrap();
    let instructions2 = read_instructions(&data2)?;
    assert_eq!(sum_correctly_ordered(&instructions2), 6260);
    assert_eq!(sum_incorrectly_ordered(&instructions1)?, 123);
    assert_eq!(sum_incorrectly_ordered(&instructions2)?, 5346);
    Ok(())
}
