// Copyright (C) 2018-2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::common;

#[test]
fn max_size_in_level() {
    let test_data = vec![(0, 1), (1, 2), (2, 4), (3, 8), (4, 16), (5, 32), (6, 64)];
    for (input, expected) in test_data {
        assert_eq!(expected, common::max_size_in_level(input));
    }
}
