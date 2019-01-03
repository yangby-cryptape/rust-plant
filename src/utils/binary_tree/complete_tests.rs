// Copyright (C) 2018-2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use super::complete;

#[test]
fn count_leaves_by_nodes() {
    let test_data = vec![
        (0, 0),
        (1, 1),
        (2, 1),
        (3, 2),
        (4, 2),
        (5, 3),
        (11, 6),
        (12, 6),
        (13, 7),
    ];
    for (input, expected) in test_data {
        assert_eq!(expected, complete::count_leaves_by_nodes(input));
    }
}

#[test]
fn is_full_by_nodes() {
    let test_data = vec![
        (true, vec![0, 1, 3, 5, 7, 9, 11]),
        (false, vec![2, 4, 6, 8, 10, 12]),
    ];
    for (expected, inputs) in test_data {
        for input in inputs {
            assert_eq!(expected, complete::is_full_by_nodes(input));
        }
    }
}

#[test]
fn is_perfect_by_nodes() {
    let test_data = vec![
        (true, vec![0, 1, 3, 7, 15]),
        (false, vec![2, 4, 5, 6, 8, 9, 10, 11, 12, 13, 14, 16]),
    ];
    for (expected, inputs) in test_data {
        for input in inputs {
            assert_eq!(expected, complete::is_perfect_by_nodes(input));
        }
    }
}

#[test]
fn max_level_by_nodes() {
    let test_data = vec![
        (1, vec![1]),
        (2, vec![2, 3]),
        (3, vec![4, 5, 6, 7]),
        (4, vec![8, 9, 10, 11, 12, 13, 14, 15]),
        (5, vec![16, 17, 18]),
    ];
    for (expected, inputs) in test_data {
        for input in inputs {
            assert_eq!(expected, complete::max_level_by_nodes(input));
        }
    }
}

#[test]
fn first_index_in_level() {
    let test_data = vec![(1, 0), (2, 1), (3, 3), (4, 7), (5, 15), (6, 31), (7, 63)];
    for (input, expected) in test_data {
        assert_eq!(expected, complete::first_index_in_level(input));
    }
}

#[test]
#[should_panic]
fn first_index_in_level_panic() {
    let _ = complete::first_index_in_level(0);
}

#[test]
fn find_sibling() {
    let test_data = vec![(1, 2), (3, 4), (5, 6), (7, 8), (9, 10)];
    for (left, right) in test_data {
        assert_eq!(right, complete::find_sibling(left));
        assert_eq!(left, complete::find_sibling(right));
    }
}

#[test]
#[should_panic]
fn find_sibling_panic() {
    let _ = complete::find_sibling(0);
}

#[test]
fn find_parent() {
    let test_data = vec![
        (0, vec![1, 2]),
        (1, vec![3, 4]),
        (2, vec![5, 6]),
        (3, vec![7, 8]),
        (4, vec![9, 10]),
        (5, vec![11, 12]),
        (15, vec![31]),
    ];
    for (expected, inputs) in test_data {
        for input in inputs {
            assert_eq!(expected, complete::find_parent(input));
        }
    }
}

#[test]
fn find_children() {
    let test_data = vec![
        (0, (1, 2)),
        (1, (3, 4)),
        (2, (5, 6)),
        (3, (7, 8)),
        (4, (9, 10)),
        (5, (11, 12)),
        (6, (13, 14)),
    ];
    for (input, expected) in test_data {
        assert_eq!(expected, complete::find_children(input));
    }
}

#[test]
#[should_panic]
fn find_parent_panic() {
    let _ = complete::find_parent(0);
}

#[test]
fn find_path_from_root() {
    let test_data = vec![
        (1, vec![0, 1]),
        (2, vec![0, 2]),
        (3, vec![0, 1, 3]),
        (4, vec![0, 1, 4]),
        (5, vec![0, 2, 5]),
        (6, vec![0, 2, 6]),
        (31, vec![0, 1, 3, 7, 15, 31]),
    ];
    for (input, expected) in test_data {
        assert_eq!(expected, complete::find_path_from_root(input));
    }
}
