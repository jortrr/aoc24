use crate::{
    file::read_file_by_line,
    parse::parse_as,
    solver::{self, Int, Solution},
};
use std::{cmp::Ordering, error::Error, fmt::Debug};

type Update = Vec<Int>;

struct Rule {
    before: Int,
    after: Int,
}

impl Rule {
    fn parse(s: &str) -> Result<Rule, Box<dyn Error>> {
        let numbers: Vec<Int> =
            parse_as::<Int>(s, "|")?;
        Ok(Rule {
            before: *numbers
                .get(0)
                .ok_or("Missing before")?,
            after: *numbers
                .get(1)
                .ok_or("Missing after")?,
        })
    }
}

impl Debug for Rule {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}|{}", self.before, self.after)
    }
}

fn order_update(
    update: &Update,
    rules: &Vec<&Rule>,
) -> Update {
    println!("Ordering update: {:?}", update);
    println!("Rules: {:?}", rules);
    let mut ordered_update = update.clone();
    ordered_update.sort_by(|a, b| {
        if rules
            .iter()
            .any(|r| r.before == *a && r.after == *b)
        {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    println!("Ordered update: {:?}", ordered_update);
    println!();
    ordered_update
}

pub struct Solver;

impl solver::Solver for Solver {
    fn run() -> Result<Solution, Box<dyn Error>> {
        let file: Vec<String> =
            read_file_by_line("05.txt")?;
        let file_sections: Vec<Vec<String>> = file
            .split(|line| line.is_empty())
            .map(|group| group.to_vec())
            .collect();
        let mut rules: Vec<Rule> = file_sections
            .get(0)
            .ok_or("Missing rules section")?
            .iter()
            .map(|s| Rule::parse(s))
            .collect::<Result<Vec<_>, _>>()?;
        rules.sort_by_key(|rule| rule.before);
        let rules = rules;
        let updates: Vec<Vec<Int>> = file_sections
            .get(1)
            .unwrap()
            .iter()
            .map(|s| parse_as::<Int>(s, ","))
            .collect::<Result<Vec<_>, _>>()?;
        let mut sum: Int = 0;
        let mut ordered_sum: Int = 0;

        for update in &updates {
            let current_rules: Vec<&Rule> = rules
                .iter()
                .filter(|rule| {
                    update.contains(&rule.before)
                        && update.contains(&rule.after)
                })
                .collect();
            let mut ordered = true;
            for page in update {
                for rule in &current_rules {
                    if *page == rule.after {
                        for other_page in update {
                            if other_page == page {
                                ordered = false;
                                break;
                            }
                            if *other_page
                                == rule.before
                            {
                                break;
                            }
                        }
                    }
                }
            }
            if ordered {
                sum += update
                    .get(update.len() / 2)
                    .ok_or("Update out of bounds")?;
            } else {
                let ordered_update: Update =
                    order_update(
                        update,
                        &current_rules,
                    );
                ordered_sum += ordered_update
                    .get(ordered_update.len() / 2)
                    .unwrap();
            }
        }

        Ok(Solution {
            part_one: sum,
            part_two: ordered_sum,
        })
    }
}
