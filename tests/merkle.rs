// Copyright (C) 2018-2019 Boyu Yang
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use plant::prelude::merkle::{IntoTree, IsData, IsHash};
use plant::utils::binary_tree::full_complete::count_nodes_by_leaves;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Hash {
    inner: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Data {
    inner: u8,
}

impl IsHash for Hash {
    fn merge(&self, rhs: &Self) -> Self {
        let mut inner = self.inner.clone();
        for i in rhs.inner.iter() {
            inner.push(*i);
        }
        Self { inner }
    }
}

impl IsData<Hash> for Data {
    fn hash(&self) -> Hash {
        let mut inner = Vec::new();
        inner.push(self.inner);
        Hash { inner }
    }
}

#[test]
fn generate_and_verify_proof() {
    let leaves_size = 32;
    let mut k = Vec::<Data>::new();
    for i in 0..(leaves_size as u8) {
        let data = Data { inner: i };
        k.push(data);
    }
    let tree = k.clone().into_tree();
    let root = tree.root();
    for j in 0..leaves_size {
        let proof = tree.generate_proof_for_leaf_index(j);
        for i in 0..(leaves_size as u8) {
            let data = Data { inner: i };
            let result = proof.verify_data(&root, &data);
            assert_eq!(i == (j as u8), result);
        }
    }
    let nodes_size = count_nodes_by_leaves(leaves_size);
    for j in 1..nodes_size {
        let proof = tree.generate_proof_for_index(j);
        for i in 0..(leaves_size as u8) {
            let data = Data { inner: i };
            let result = proof.verify_data(&root, &data);
            assert_eq!(i + ((nodes_size - leaves_size) as u8) == (j as u8), result);
        }
    }
}
