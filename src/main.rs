fn main() {
    main::Solution::two_sum(vec![1, 2, 3, 4], 5);
}

mod main {
    pub struct Solution;
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut i: usize = 0;
            let mut _curr_sum: i32 = 0;
            loop {
                if _curr_sum == target {
                    println!(
                        "returning: {:?}, {:?}",
                        vec![nums[i - 1], nums[i]],
                        vec![i - 1, i]
                    );
                    return vec![i as i32 - 1, i as i32];
                }
                if i < nums.len() - 1 {
                    _curr_sum = nums[i] + nums[i + 1];
                    i += 1;
                    continue;
                }
                break;
            }
            println!("Not found, probably not working");
            vec![1, 2]
        }
    }
}
/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.
*/
