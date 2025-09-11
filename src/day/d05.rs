use crate::{
    file::read_file_by_line,
    parse::parse_as,
    solver::{self, Int, Solution},
};
use std::error::Error;

#[derive(Debug)]
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

pub struct Solver;

impl solver::Solver for Solver {
    fn run() -> Result<Solution, Box<dyn Error>> {
        let file: Vec<String> =
            read_file_by_line("05.txt")?;
        let file_sections: Vec<Vec<String>> = file
            .split(|line| line.is_empty())
            .map(|group| group.to_vec())
            .collect();
        dbg!(&file_sections);
        let rules: Vec<Rule> = file_sections
            .get(0)
            .ok_or("Missing rules section")?
            .iter()
            .map(|s| Rule::parse(s))
            .collect::<Result<Vec<_>, _>>()?;
        let updates: Vec<Vec<Int>> = file_sections
            .get(1)
            .unwrap()
            .iter()
            .map(|s| parse_as::<Int>(s, ","))
            .collect::<Result<Vec<_>, _>>()?;
        dbg!(&rules);
        dbg!(&updates);
        let mut sum: Int = 0;

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
                dbg!(update);
                sum += update
                    .get(update.len() / 2)
                    .ok_or("Update out of bounds")?;
            }
        }
        dbg!(&sum);

        Err("TODO".into())
    }
}
