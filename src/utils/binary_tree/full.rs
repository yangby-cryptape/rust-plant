// Copyright (C) 2018-2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#[allow(clippy::collapsible_if)]
#[inline]
pub fn is_left(node_index: usize) -> bool {
    if cfg!(test) || cfg!(debug_assertions) {
        if node_index == 0 {
            panic!("root node is neither left nor night");
        }
    }
    node_index % 2 == 1
}

#[allow(clippy::collapsible_if)]
#[inline]
pub fn is_right(node_index: usize) -> bool {
    if cfg!(test) || cfg!(debug_assertions) {
        if node_index == 0 {
            panic!("root node is neither left nor night");
        }
    }
    node_index % 2 == 0
}
