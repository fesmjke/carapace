use std::cmp::Eq;
use std::collections::HashSet;
use std::hash::Hash;

pub fn unique<T: Eq + Hash>(sequence: &Vec<T>) -> usize {
    sequence
        .into_iter()
        .map(|x| x)
        .collect::<HashSet<_>>()
        .len()
}
