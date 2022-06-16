fn main() {
    main::Solution::two_sum(vec![5, 9, 1, 2, 3, 4], 5);
}

mod main {
    pub struct Solution;
    impl Solution {
        pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
            let mut i: usize = 0;
            let mut j: usize = 0;
            loop {
                if i < nums.len() {
                    let x: i32 = nums[i];
                    let y: i32 = target - x;
                    loop {
                        if j < nums.len() {
                            if y == nums[j] {
                                return vec![i as i32, j as i32];
                            }
                            j += 1;
                            continue;
                        }
                        j = 0;
                        break;
                    }
                    i += 1;
                    continue;
                }
                break;
            }
            vec![0, 0]
        }
    }
}
/*
Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

You may assume that each input would have exactly one solution, and you may not use the same element twice.

You can return the answer in any order.


steps:
1. take the target and find all the two number combos equal to the target
2. then search the vector for the numbers that make up the combos
3. return answer


*/
