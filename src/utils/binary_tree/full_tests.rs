// Copyright (C) 2018-2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::full;

#[test]
fn is_left_or_right() {
    let test_data = vec![(true, vec![1, 3, 5, 7, 9]), (false, vec![2, 4, 6, 8, 10])];
    for (is_left, inputs) in test_data {
        for input in inputs {
            if is_left {
                assert!(full::is_left(input));
                assert!(!full::is_right(input));
            } else {
                assert!(!full::is_left(input));
                assert!(full::is_right(input));
            }
        }
    }
}

#[test]
#[should_panic]
fn is_left_panic() {
    let _ = full::is_left(0);
}

#[test]
#[should_panic]
fn is_right_panic() {
    let _ = full::is_right(0);
}
