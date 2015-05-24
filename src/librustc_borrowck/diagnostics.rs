// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_snake_case)]

register_diagnostics! {
    E0373, // closure may outlive current fn, but it borrows {}, which is owned by current fn
    E0378, // use/capture of possibly uninitialized variable
    E0379, // use of partially/collaterally moved value
    E0380, // partial reinitialization of uninitialized structure
    E0381, // reassignment of immutable variable
    E0382, // {} in an aliasable location
    E0383, // {} in an immutable container
    E0384, // {} in a captured outer variable in an `Fn` closure
    E0385, // {} in a static location
    E0386  // {} in a `&` reference
}
