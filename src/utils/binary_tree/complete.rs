// Copyright (C) 2018-2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

const USIZE_BITSIZE: usize = ::std::mem::size_of::<usize>() * 8;

#[inline]
pub fn count_leaves_by_nodes(nodes_size: usize) -> usize {
    (nodes_size + 1) / 2
}

#[inline]
pub fn is_full_by_nodes(nodes_size: usize) -> bool {
    nodes_size == 0 || (nodes_size % 2 == 1)
}

#[inline]
pub fn is_perfect_by_nodes(nodes_size: usize) -> bool {
    nodes_size == 0 || (nodes_size + 1).count_ones() == 1
}

#[inline]
pub fn max_level_by_nodes(nodes_size: usize) -> usize {
    if nodes_size == 0 {
        0
    } else {
        USIZE_BITSIZE - (nodes_size.leading_zeros() as usize)
    }
}

#[allow(clippy::collapsible_if)]
#[inline]
pub fn first_index_in_level(level: usize) -> usize {
    if cfg!(test) || cfg!(debug_assertions) {
        if level == 0 {
            panic!("the minimum level is 1.");
        }
    }
    (1 << (level - 1)) - 1
}

#[allow(clippy::collapsible_if)]
#[inline]
pub fn find_sibling(node_index: usize) -> usize {
    if cfg!(test) || cfg!(debug_assertions) {
        if node_index == 0 {
            panic!("the root node doesn't have any sibling node.");
        }
    }
    if node_index % 2 == 0 {
        node_index - 1
    } else {
        node_index + 1
    }
}

#[allow(clippy::collapsible_if)]
#[inline]
pub fn find_parent(node_index: usize) -> usize {
    if cfg!(test) || cfg!(debug_assertions) {
        if node_index == 0 {
            panic!("the root node doesn't have any parent node.");
        }
    }
    (node_index - 1) / 2
}

#[inline]
pub fn find_children(node_index: usize) -> (usize, usize) {
    let tmp = node_index * 2;
    (tmp + 1, tmp + 2)
}

#[inline]
pub fn find_path_from_root(node_index: usize) -> Vec<usize> {
    if node_index == 0 {
        vec![0]
    } else {
        let level = max_level_by_nodes(node_index + 1);
        let mut ret = Vec::with_capacity(level);
        let node_index_in_math = node_index + 1;
        for level in (0..level).rev() {
            ret.push((node_index_in_math >> level) - 1);
        }
        debug_assert_eq!(ret.len(), level);
        ret
    }
}
