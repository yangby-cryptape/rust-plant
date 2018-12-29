// Copyright (C) 2018-2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::utils::binary_tree::{common, complete, full, full_complete};

pub trait IsHash
where
    Self: ::std::cmp::Eq,
    Self: ::std::default::Default,
    Self: ::std::clone::Clone,
    Self: ::std::fmt::Debug,
{
    fn merge(&self, rhs: &Self) -> Self;
}

pub trait IsData<H>
where
    H: IsHash,
{
    fn hash(&self) -> H;
}

#[derive(Debug, Clone)]
pub struct Tree<H>
where
    H: IsHash,
{
    nodes: Vec<H>,
}

impl<H> Tree<H>
where
    H: IsHash,
{
    #[inline]
    fn calc_hash_for_range(&mut self, range: ::std::ops::Range<usize>) {
        for parent_index in range {
            let (left_index, right_index) = complete::find_children(parent_index);
            let left = &self.nodes[left_index];
            let right = &self.nodes[right_index];
            let parent = left.merge(right);
            self.nodes[parent_index] = parent;
        }
    }

    #[inline]
    fn calc_hash_for_level(&mut self, level: usize) {
        let range = ::std::ops::Range {
            start: complete::first_index_in_level(level),
            end: complete::first_index_in_level(level + 1),
        };
        self.calc_hash_for_range(range);
    }

    #[inline]
    pub fn generate_proof_for_index(&self, node_index: usize) -> Proof<H> {
        let path = complete::find_path_to_root(node_index);
        let mut nodes = Vec::new();
        {
            let index = complete::find_sibling(node_index);
            let data = self.nodes[index].clone();
            let node = if full::is_left(index) {
                Node::Left(data)
            } else {
                Node::Right(data)
            };
            nodes.push(node);
        }
        for index in path.into_iter() {
            let index = complete::find_sibling(index);
            let data = self.nodes[index].clone();
            let node = if full::is_left(index) {
                Node::Left(data)
            } else {
                Node::Right(data)
            };
            nodes.push(node);
        }
        Proof { nodes }
    }

    #[inline]
    pub fn generate_proof_for_leaf_index(&self, leaf_index: usize) -> Proof<H> {
        let nodes_size = self.nodes.len();
        let leaves_size = complete::count_leaves_by_nodes(nodes_size);
        let node_index = nodes_size - leaves_size + leaf_index;
        self.generate_proof_for_index(node_index)
    }

    #[inline]
    pub fn root(&self) -> &H {
        &self.nodes[0]
    }
}

impl<H, D> IntoTree<H> for Vec<D>
where
    D: IsData<H>,
    H: IsHash,
{
    #[inline]
    fn size(&self) -> usize {
        self.len()
    }
}

pub trait IntoTree<H>
where
    H: IsHash,
    Self: std::marker::Sized,
    Self: ::std::iter::IntoIterator,
    <Self as ::std::iter::IntoIterator>::Item: IsData<H>,
{
    fn size(&self) -> usize;

    #[inline]
    fn into_tree(self) -> Tree<H> {
        let leaves_size = self.size();
        if leaves_size == 0 {
            Tree {
                nodes: vec![H::default(); 1],
            }
        } else {
            let nodes_size = full_complete::count_nodes_by_leaves(leaves_size);
            let mut max_level_undo = complete::max_level_by_nodes(nodes_size);
            let mut tree = Tree {
                nodes: vec![H::default(); nodes_size],
            };
            let first_leaf_index = nodes_size - leaves_size;
            for (leaf_index, leaf_data) in self.into_iter().enumerate() {
                let leaf_hash = leaf_data.hash();
                tree.nodes[first_leaf_index + leaf_index] = leaf_hash;
            }
            if max_level_undo > 0 {
                let max_size_at_last_level = common::max_size_in_level(max_level_undo);
                if leaves_size != max_size_at_last_level {
                    max_level_undo -= 1;
                    let range = ::std::ops::Range {
                        start: complete::first_index_in_level(max_level_undo),
                        end: first_leaf_index,
                    };
                    tree.calc_hash_for_range(range);
                }
                while max_level_undo != 0 {
                    max_level_undo -= 1;
                    tree.calc_hash_for_level(max_level_undo);
                }
            }
            tree
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Node<H>
where
    H: IsHash,
{
    Left(H),
    Right(H),
}

impl<H> Node<H>
where
    H: IsHash,
{
    pub fn is_left(&self) -> bool {
        if let Node::Left(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn is_right(&self) -> bool {
        if let Node::Right(_) = *self {
            true
        } else {
            false
        }
    }

    pub fn merge(&self, that: &H) -> H {
        match self {
            Node::Left(ref this) => this.merge(that),
            Node::Right(ref this) => that.merge(this),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Proof<H>
where
    H: IsHash,
{
    nodes: Vec<Node<H>>,
}

impl<H> Proof<H>
where
    H: IsHash,
{
    pub fn verify_data<D>(&self, root_hash: &H, data: &D) -> bool
    where
        D: IsData<H>,
    {
        let data_hash = data.hash();
        self.verify(root_hash, &data_hash)
    }

    pub fn verify(&self, root_hash: &H, data_hash: &H) -> bool {
        (&self
            .nodes
            .iter()
            .fold(data_hash.clone(), |h, ref x| x.merge(&h)))
            == root_hash
    }
}
