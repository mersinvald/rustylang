#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json as json;
extern crate toml;
extern crate regex;
#[macro_use]
extern crate failure;

use failure::Error;
use regex::Regex;

const RULES_TO_RUSTY: &str = include_str!("../to_rusty.json");

pub fn to_rusty(input: &str) -> String {
    let transformer = Transformer::load_rules(RULES_TO_RUSTY);
    unimplemented!()
}

#[derive(Debug, Clone)]
struct Rule {
    regex: regex::Regex,
    subst: String,
}

#[derive(Deserialize, Debug, Clone)]
struct LoadedRule {
    regex: String,
    subst: String,
}

struct Transformer {
    rules: Vec<Rule>,
}

use std::path::Path;
use std::io;
use std::fs;
use std::str::FromStr;

impl Transformer {
    pub fn load_rules(rules: &str) -> Self {
        let rules: Vec<LoadedRule> = json::from_str(rules)
            .expect("Rules file is an invalid JSON");

        let (rules, failed): (Vec<_>, Vec<_>) = rules.into_iter()
            .map(|rule| {
                Regex::from_str(&rule.regex)
                    .map(|regex| Rule {
                        regex,
                        subst: rule.subst
                    })
            })
            .partition(Result::is_ok);

        if !failed.is_empty() {
            Self::print_failed_and_panic(failed)
        }

        Transformer {
            rules: rules.into_iter().map(Result::unwrap).collect()
        }
    }

    pub fn transform(&self, text: &str) -> String {
        let mut out = text.to_string();
        for rule in &self.rules {
            out = rule.regex.replace_all(&out, rule.subst.as_str()).to_string();
        }
        out
    }

    fn print_failed_and_panic(failed: Vec<Result<Rule, regex::Error>>) -> ! {
        eprintln!("Failed to load rules:");
        for fail in failed {
            eprintln!("Failed to load rule: {:?}", fail.unwrap_err());
        }
        panic!("Rules file is invalid");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_rules_to_rusty() {
        Transformer::load_rules(RULES_TO_RUSTY);
    }

    #[test]
    fn transform_to_rusty() {
        let transformer = Transformer::load_rules(RULES_TO_RUSTY);
        let text = r#"fn main() { println!("Hello, World") }"#;
        let out = transformer.transform(text);
        println!("{}", out);
        unimplemented!();
    }
}
