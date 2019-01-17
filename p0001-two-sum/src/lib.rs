// Given an array of integers, return indices of the two numbers such that they add up to a specific target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.
use std::collections::HashMap;

pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut required = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            match required.get(&(target - n)) {
                Some(first) => return vec![*first as i32, i as i32],
                None => required.insert(n, i),
            };
        }

        vec![]
    }
}
