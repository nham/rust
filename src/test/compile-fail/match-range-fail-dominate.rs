// Copyright 2012-2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn main() {
    match 5 {
      1 ... 10 => { }
      5 ... 6 => { } //~ ERROR unreachable pattern
      _ => {}
    };

    match 5 {
      3 ... 6 => { }
      4 ... 6 => { } //~ ERROR unreachable pattern
      _ => {}
    };

    match 5 {
      4 ... 6 => { }
      4 ... 6 => { } //~ ERROR unreachable pattern
      _ => {}
    };

    match 'c' {
      'A' ... 'z' => {}
      'a' ... 'z' => {} //~ ERROR unreachable pattern
      _ => {}
    };

    match 1.0f64 {
      0.01f64 ... 6.5f64 => {}
      0.02f64 => {} //~ ERROR unreachable pattern
      _ => {}
    };
}
