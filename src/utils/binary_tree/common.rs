// Copyright (C) 2018-2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(clippy::collapsible_if)]
#[inline]
pub fn max_size_in_level(level: usize) -> usize {
    if cfg!(test) || cfg!(debug_assertions) {
        if level == 0 {
            panic!("the minimum level is 1.");
        }
    }
    1 << (level - 1)
}
