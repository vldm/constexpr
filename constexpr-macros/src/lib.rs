// Copyright (c) 2018 Vladimir Motylenko
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

#[macro_use]
extern crate proc_macro_hack;
#[cfg_attr(test, macro_use)]
extern crate pest;
#[macro_use]
extern crate pest_derive;

mod grammar;
//TODO: Add support of public methods.

proc_macro_item_impl! {
    pub fn template_impl(input: &str) -> String {
        // TODO: Add support of rust patterns in arguments.
        let body_start = input.find("{").expect("Couldn't find function body.");
        let (signature, body) = input.split_at(body_start);
        let result = grammar::parse_signature(signature);
        let macro_name = result.macro_name();
        let macro_signature = result.to_macro_signature();
        let binding = result.to_binding();
        let out = format!("macro_rules! {} {{({}) =>\
                        {{ {{ {} {} }} }} \
                }}", macro_name, macro_signature, binding, body);
//        println!("out = {}", out);
        out
    }
}


