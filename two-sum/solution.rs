use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sums(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut complements: HashMap<i32, i32> = HashMap::new();
        
        for (index, item) in nums.iter().enumerate() {
            match complements.get(&(target - item)) {
                Some(result) => return vec![*result, index as i32],
                None => {
                    complements.insert(*item, index as i32);
                    ()
                }
            }
        }

        nums
    }
}

fn main() {
    let nums: Vec<i32> = [2,7,11,15].to_vec();
    let target: i32 = 9;
    println!("{:?}", Solution::two_sums(nums, target));
}