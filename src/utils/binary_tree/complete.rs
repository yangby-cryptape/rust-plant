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
        USIZE_BITSIZE - 1 - (nodes_size.leading_zeros() as usize)
    }
}

#[inline]
pub fn first_index_in_level(level: usize) -> usize {
    (1 << level) - 1
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
pub fn find_path_to_root(mut node_index: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    if node_index != 0 {
        node_index = (node_index - 1) / 2;
        while node_index != 0 {
            ret.push(node_index);
            node_index = (node_index - 1) / 2;
        }
    }
    ret
}
