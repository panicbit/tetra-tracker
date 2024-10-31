use std::fmt;
use std::str::FromStr;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::Parser;
use eyre::eyre;
use serde::de;
use serde::Deserialize;
use serde::Serialize;

pub mod parser;

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Rule {
    /// rule1,rule1,…
    Multi(Vec<Rule>),
    /// item_name
    Item(String),
    /// $fn_name|arg1|arg2|…
    Call(Call),
    /// ^$fn_name|arg1|arg2|…
    AccessabilityLevel(Call),
    /// @location/section
    Reference(Reference),
    /// { rule }
    Checkable(Box<Rule>),
    /// [ rule ]
    Optional(Box<Rule>),
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rule::Multi(vec) => {
                for (i, rule) in vec.iter().enumerate() {
                    if i == 0 {
                        write!(f, "{rule}")?;
                    } else {
                        write!(f, ",{rule}")?;
                    }
                }
            }
            Rule::Item(item) => item.fmt(f)?,
            Rule::Call(call) => {
                let Call { name, args } = call;

                write!(f, "${name}")?;

                for arg in args {
                    write!(f, "|{arg}")?;
                }
            }
            Rule::AccessabilityLevel(call) => {
                let Call { name, args } = call;

                write!(f, "^${name}")?;

                for arg in args {
                    write!(f, "|{arg}")?;
                }
            }
            Rule::Reference(reference) => {
                let Reference { location, section } = reference;
                write!(f, "@{location}/{section}")?;
            }
            Rule::Checkable(rule) => write!(f, "{{{rule}}}")?,
            Rule::Optional(rule) => write!(f, "[{rule}]")?,
        }

        Ok(())
    }
}

impl FromStr for Rule {
    type Err = eyre::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let (rule, errs) = parser::rule().parse(input).into_output_errors();

        if let Some(rule) = rule {
            return Ok(rule);
        }

        let mut out = Vec::new();

        for err in errs {
            let span = err.span().into_range();

            Report::build(ReportKind::Error, span)
                .with_message(err.to_string())
                .with_label(
                    Label::new(err.span().into_range())
                        .with_message(err.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .write(Source::from(&input), &mut out)
                .unwrap();
        }

        let out = String::from_utf8_lossy(&out).into_owned();

        Err(eyre!(out))
    }
}

impl<'de> Deserialize<'de> for Rule {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let rule = String::deserialize(de)?;
        let rule = rule.parse::<Rule>().map_err(de::Error::custom)?;

        Ok(rule)
    }
}

impl Serialize for Rule {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Call {
    pub name: String,
    pub args: Vec<String>,
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Reference {
    pub location: String,
    pub section: String,
}
