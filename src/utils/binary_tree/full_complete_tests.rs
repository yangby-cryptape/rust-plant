// Copyright (C) 2018-2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::full_complete;

#[test]
fn count_nodes_by_leaves() {
    let test_data = vec![
        (1, 1),
        (2, 3),
        (3, 5),
        (4, 7),
        (5, 9),
        (6, 11),
        (7, 13),
        (8, 15),
    ];
    for (input, expected) in test_data {
        assert_eq!(expected, full_complete::count_nodes_by_leaves(input));
    }
}
