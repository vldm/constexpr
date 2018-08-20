// Copyright (c) 2018 Vladimir Motylenko
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.


#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate constexpr_macros;
#[doc(hidden)]
pub use constexpr_macros::*;

proc_macro_item_decl! {
    template! => template_impl
}