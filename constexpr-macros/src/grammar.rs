// Copyright (c) 2018 Vladimir Motylenko
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[cfg(debug_assertions)]
const _GRAMMAR: &'static str = include_str!("../templates.pest");

#[derive(Parser)]
#[grammar = "../templates.pest"]
pub struct TemplateFn;

use pest::Parser;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug)]
pub struct Argument<'a> {
    name: &'a str,
    type_def: &'a str,
}
#[derive(Eq, PartialEq, Debug)]
pub struct ReturnType<'a > {
    pub macro_name: & 'a str,
    pub generics: HashSet<& 'a str>,
    pub args: Vec<Argument<'a>>,
    pub return_param: Option<&'a str>
}

impl<'a> ReturnType<'a> {

    pub fn macro_name(&self) -> &'a str {
        self.macro_name
    }

    #[allow(dead_code)]
    pub fn to_closure_signature(&self) -> String {
        let mut result = String::from("|");
        for arg in &self.args {
            result.push_str(arg.name);
            result.push_str(":");
            if self.generics.contains(arg.type_def) {
                result.push_str("_");
            }
            else {
                result.push_str(arg.type_def);
            }
            result.push_str(",");
        }
        result.push_str("|");
        match self.return_param {
            Some(result_type) if self.generics.contains(result_type) => {},
            Some(result_type) => {
                result.push_str("->");
                result.push_str(result_type);
            }
            _ => {}
        }
        result
    }

    pub fn to_macro_signature(&self) -> String {
        let mut result = String::new();
        let mut args_iter = self.args.iter();
        if let Some(first) = args_iter.next() {
            result.push_str("$");
            result.push_str(first.name);
            result.push_str(":expr");
        }
        for arg in args_iter {
            result.push_str(",$");
            result.push_str(arg.name);
            result.push_str(":expr");
        }
        result
    }
    pub fn to_binding(&self) -> String {
        let mut result = String::new();
        for arg in self.args.iter() {
            result.push_str("let ");
            result.push_str(arg.name);
            result.push_str("=$");
            result.push_str(arg.name);
            result.push_str(";");
        }
        result
    }
}

pub fn parse_signature(input: &str) -> ReturnType {
    let mut macro_generate = TemplateFn::parse(Rule::function, &input)
        .expect("Could not parse method signature.");
    let mut signature = macro_generate.next().unwrap().into_inner();
    let macro_name = signature.next().unwrap().as_str();
    let generics = signature.next().unwrap();
    let arguments = signature.next().unwrap();
    let return_param = signature.next().map(|p|{
        p.as_str()
    });
    let args = arguments.into_inner().map(|p|p.into_inner())
        .map(|mut p| (p.next().unwrap(), p.next().unwrap()))
        .map(|(name, type_def)| Argument {
            name: name.as_str(),
            type_def: type_def.as_str()
        })
        .collect();
    ReturnType {
        macro_name,
        generics: generics.into_inner().map(|p|p.as_str()).collect(),
        args,
        return_param,
    }
}

#[cfg(test)]
mod test {
    use super::TemplateFn;
    use super::Rule;
    use super::{ReturnType, Argument, parse_signature};
    #[test]
    fn identifier_simple() {
        parses_to! {
            parser: TemplateFn,
            input: "name",
            rule: Rule::identifier,
            tokens: [
                identifier(0, 4)
            ]
        };
    }

    #[test]
    fn identifier_with_seperator() {
        parses_to! {
            parser: TemplateFn,
            input: "other_name",
            rule: Rule::identifier,
            tokens: [
                identifier(0, 10)
            ]
        };
    }


    #[test]
    fn type_def_simple() {
        parses_to! {
            parser: TemplateFn,
            input: "u32",
            rule: Rule::type_def,
            tokens: [
                type_def(0, 3)
            ]
        };
    }

    #[test]
    #[ignore]
    fn type_def_generic() {
        parses_to! {
            parser: TemplateFn,
            input: "Map<T, X>",
            rule: Rule::type_def,
            tokens: [
                type_def(0, 9)
            ]
        };
    }

    #[test]
    fn pat_simple() {
        parses_to! {
            parser: TemplateFn,
            input: "var_name",
            rule: Rule::pat,
            tokens: [
                pat(0, 8,[
                    identifier(0, 8)
                ])
            ]
        };
    }

    #[test]
    fn pat_tuple() {
        parses_to! {
            parser: TemplateFn,
            input: "(tuple, other)",
            rule: Rule::pat,
            tokens: [
                pat(0, 14,[
                    pat(1, 6, [identifier(1, 6)]),
                    pat(8, 13, [identifier(8, 13)])
                ])
            ]
        };
    }

    #[test]
    fn argument() {
        parses_to! {
            parser: TemplateFn,
            input: "ident: Type",
            rule: Rule::argument,
            tokens: [
                argument(0, 11,[
                    pat(0, 5, [identifier(0, 5)]),
                    type_def(7, 11),
                ])
            ]
        };
    }

    #[test]
    fn signature_simple() {
        parses_to! {
            parser: TemplateFn,
            input: "fn my_generic<T>(tuple: T)",
            rule: Rule::signature,
            tokens: [
                signature(0, 26,[
                    identifier(3, 13),
                    generics(14,15, [identifier(14, 15)]),
                    arguments(17, 25, [
                        argument(17, 25, [pat(17, 22, [identifier(17, 22)]),
                                        type_def(24, 25)])
                        ]),

                ])
            ]
        };
    }

    #[test]
    fn signature_multiple() {
        parses_to! {
            parser: TemplateFn,
            input: "fn my_generic<T, X>(tuple: T, other:Y)",
            rule: Rule::signature,
            tokens: [
                signature(0, 38,[
                    identifier(3, 13),
                    generics(14, 18, [identifier(14, 15), identifier(17, 18),]),
                    arguments(20, 37, [
                        argument(20, 28, [pat(20, 25, [identifier(20, 25)]),
                                        type_def(27, 28)]),
                        argument(30, 37, [pat(30, 35, [identifier(30, 35)]),
                                        type_def(36, 37)])
                        ]),

                ])
            ]
        };
    }

    #[test]
    fn test_parse_full_signature() {
        let input = "fn my_generic<T>(tuple: T)";
        let output = ReturnType {
                macro_name: "my_generic",
                generics: vec!["T"].into_iter().collect(),
                return_param: None,
                args:vec![Argument {
                    name: "tuple",
                    type_def:"T"
                }]
        };
        assert_eq!(parse_signature(input), output);
    }

    #[test]
    fn test_generate_closure() {
        let input = "fn my_generic<T>(tuple: T, other:u32) -> u32";
        let output = "|tuple:_,other:u32,|->u32";

        assert_eq!(parse_signature(input).to_closure_signature(), output);
    }

    #[test]
    fn test_generate_macro() {
        let input = "fn my_generic<T>(tuple: T, other:u32) -> u32";
        let output = "$tuple:expr,$other:expr";

        assert_eq!(parse_signature(input).to_macro_signature(), output);
    }

    #[test]
    fn test_generate_binding() {
        let input = "fn my_generic<T>(tuple: T, other:u32) -> u32";
        let output = "let tuple=$tuple;let other=$other;";

        assert_eq!(parse_signature(input).to_binding(), output);
    }
}


