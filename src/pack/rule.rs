use std::fmt;
use std::str::FromStr;

use ariadne::{Color, Label, Report, ReportKind, Source};
use chumsky::Parser;
use eyre::eyre;
use mlua::FromLua;
use mlua::Function;
use mlua::IntoLua;
use mlua::Lua;
use mlua::MultiValue;
use serde::de;
use serde::Deserialize;
use serde::Serialize;

use crate::pack::api::AccessibilityLevel;

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
    AccessibilityLevel(Call),
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
            Rule::AccessibilityLevel(call) => {
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

impl Call {
    pub fn exec<R>(&self, lua: &Lua) -> mlua::Result<R>
    where
        R: FromLua,
    {
        let fun = lua.globals().get::<Function>(self.name.as_str())?;
        let args = self.lua_args(lua)?;

        fun.call::<R>(args)
    }

    pub fn lua_args(&self, lua: &Lua) -> mlua::Result<MultiValue> {
        let mut args = MultiValue::with_capacity(self.args.len());

        for arg in &self.args {
            let lua_arg = arg.as_str().into_lua(lua)?;

            args.push_front(lua_arg);
        }

        Ok(args)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Reference {
    pub location: String,
    pub section: String,
}

#[derive(Default)]
pub struct AndCombiner {
    contains_none: bool,
    inspectable: bool,
    sequence_breakable: bool,
}

impl AndCombiner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, level: AccessibilityLevel) -> &mut Self {
        match level {
            AccessibilityLevel::None => self.contains_none = true,
            AccessibilityLevel::Partial => self.contains_none = true,
            AccessibilityLevel::Inspect => self.inspectable = true,
            AccessibilityLevel::SequenceBreak => self.sequence_breakable = true,
            AccessibilityLevel::Normal => {}
            AccessibilityLevel::Cleared => {}
        }

        self
    }

    pub fn finish(&self) -> AccessibilityLevel {
        if self.contains_none {
            return AccessibilityLevel::None;
        }

        if self.sequence_breakable {
            return AccessibilityLevel::SequenceBreak;
        }

        if self.inspectable {
            return AccessibilityLevel::Inspect;
        }

        AccessibilityLevel::Normal
    }
}

#[derive(Default)]
pub struct OrCombiner {
    inspectable: bool,
    sequence_breakable: bool,
    // TODO: maybe track if any Normal/Cleared level got found?
}

impl OrCombiner {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, level: AccessibilityLevel) -> &mut Self {
        match level {
            AccessibilityLevel::None => {}
            AccessibilityLevel::Partial => {}
            AccessibilityLevel::Inspect => self.inspectable = true,
            AccessibilityLevel::SequenceBreak => self.sequence_breakable = true,
            AccessibilityLevel::Normal => {}
            AccessibilityLevel::Cleared => {}
        }

        self
    }

    pub fn finish(&self) -> AccessibilityLevel {
        if self.sequence_breakable {
            return AccessibilityLevel::SequenceBreak;
        }

        if self.inspectable {
            return AccessibilityLevel::Inspect;
        }

        AccessibilityLevel::Normal
    }
}
