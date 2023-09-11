use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

pub fn unique<T: Eq + Hash>(sequence: &Vec<T>) -> usize {
    // TODO: implement a unique number of elements in vector
    sequence.into_iter().map(|x| x).collect::<HashSet<_>>().len()
}
