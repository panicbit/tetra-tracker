use chumsky::prelude::*;
use chumsky::text::ascii::ident;
use chumsky::Parser as _;

use crate::pack::rule::{Call, Reference, Rule};

pub trait Parser<'a, T>: chumsky::Parser<'a, &'a str, T, extra::Err<Rich<'a, char>>> {}

impl<'a, T, P> Parser<'a, T> for P where
    P: chumsky::Parser<'a, &'a str, T, extra::Err<Rich<'a, char>>>
{
}

pub fn rule<'a>() -> impl Parser<'a, Rule> {
    recursive(|rule| {
        let string = none_of("|,}]").repeated().to_slice();
        let arg = just('|').ignore_then(string);
        let args = arg.map(String::from).repeated().collect();

        let call = ident()
            .then(args)
            .map(|(name, args): (&str, Vec<String>)| Call {
                name: name.into(),
                args,
            });
        let rule_call = just("$").ignore_then(call).map(Rule::Call);
        let rule_accessibility_level = just("^$").ignore_then(call).map(Rule::AccessibilityLevel);

        let reference = none_of("/")
            .repeated()
            .to_slice()
            .then_ignore(just('/'))
            .then(string)
            .map(|(location, section): (&str, &str)| Reference {
                location: location.into(),
                section: section.into(),
            });
        let rule_reference = just("@").ignore_then(reference).map(Rule::Reference);

        let rule_checkable = rule
            .clone()
            .delimited_by(just('{'), just('}'))
            .map(Box::new)
            .map(Rule::Checkable);

        let rule_optional = rule
            .clone()
            .delimited_by(just('['), just(']'))
            .map(Box::new)
            .map(Rule::Optional);

        let rule_item = string
            .filter(|item: &&str| !item.is_empty())
            .map(String::from)
            .map(Rule::Item);

        choice((
            rule_call,
            rule_accessibility_level,
            rule_reference,
            rule_checkable,
            rule_optional,
            rule_item,
        ))
        .separated_by(just(','))
        .collect()
        .map(|mut rules: Vec<Rule>| {
            if rules.len() == 1 {
                rules.pop().unwrap()
            } else {
                Rule::Multi(rules)
            }
        })
    })
    .then_ignore(end())
}

#[cfg(test)]
mod tests {
    use super::test_helpers::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn call_without_args() {
        assert_eq!(parse("$foo"), call("foo", []));
    }

    #[test]
    fn call_with_args() {
        assert_eq!(parse("$foo|a b||42"), call("foo", ["a b", "", "42"]));
    }

    #[test]
    fn checkable_call_without_args() {
        assert_eq!(parse("{$foo}"), checkable(call("foo", [])));
    }

    #[test]
    fn checkable_call_with_args() {
        assert_eq!(parse("{$foo|arg1}"), checkable(call("foo", ["arg1"])));
    }

    #[test]
    fn checkable_reference() {
        assert_eq!(
            parse("{@location/section}"),
            checkable(reference("location", "section")),
        );
    }

    #[test]
    fn checkable_empty() {
        assert_eq!(parse("{}"), checkable(multi([])));
    }

    #[test]
    fn optional_empty() {
        assert_eq!(parse("[]"), optional(multi([])));
    }

    #[test]
    fn multi_item() {
        assert_eq!(parse("item1,item2"), multi([item("item1"), item("item2")]))
    }

    #[test]
    fn multi_checkable() {
        assert_eq!(
            parse("{@foo/bar,$call|me}"),
            checkable(multi([reference("foo", "bar"), call("call", ["me"]),])),
        );
    }

    #[test]
    fn multi_optional() {
        assert_eq!(
            parse("[@foo/bar,$call|me]"),
            optional(multi([reference("foo", "bar"), call("call", ["me"]),])),
        );
    }

    #[test]
    fn multi_checkable_and_optional() {
        assert_eq!(
            parse("{@foo/bar,$call|me},[$hello|world,@x/y]"),
            multi([
                checkable(multi([reference("foo", "bar"), call("call", ["me"]),])),
                optional(multi([call("hello", ["world"]), reference("x", "y"),]))
            ])
        );
    }

    #[test]
    fn single_reference() {
        assert_eq!(parse("@location/section"), reference("location", "section"));
    }

    #[test]
    fn single_item() {
        assert_eq!(parse("item"), item("item"));
    }
}

#[cfg(test)]
pub mod test_helpers {
    use ariadne::{Color, Label, Report, ReportKind, Source};
    use chumsky::Parser as _;

    use super::{rule, Call, Reference, Rule};

    pub fn checkable(rule: Rule) -> Rule {
        Rule::Checkable(Box::new(rule))
    }

    pub fn reference(location: &str, section: &str) -> Rule {
        Rule::Reference(Reference {
            location: location.into(),
            section: section.into(),
        })
    }

    pub fn call<'a>(name: &str, args: impl IntoIterator<Item = &'a str>) -> Rule {
        Rule::Call(Call {
            name: name.into(),
            args: args.into_iter().map(String::from).collect(),
        })
    }

    pub fn item(name: &str) -> Rule {
        Rule::Item(name.into())
    }

    pub fn optional(rule: Rule) -> Rule {
        Rule::Optional(Box::new(rule))
    }

    pub fn multi(rules: impl IntoIterator<Item = Rule>) -> Rule {
        Rule::Multi(rules.into_iter().collect())
    }

    #[track_caller]
    pub fn parse(input: &str) -> Rule {
        let (rule, errs) = rule().parse(input).into_output_errors();

        errs.into_iter().for_each(|e| {
            let span = e.span().into_range();

            let mut out = Vec::new();

            Report::build(ReportKind::Error, span)
                .with_message(e.to_string())
                .with_label(
                    Label::new(e.span().into_range())
                        .with_message(e.reason().to_string())
                        .with_color(Color::Red),
                )
                .finish()
                .write(Source::from(&input), &mut out)
                .unwrap();

            let out = String::from_utf8(out).unwrap();

            eprintln!("{out}");
        });

        rule.expect("parse failed")
    }
}
