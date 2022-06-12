fn main() {
    main::Solution::two_sum(vec![1, 2, 3, 4], 4);
    main::Solution::two_addends();
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
        // I am planning on using this function to figure out how I will get all the addends of a given number. From there, I can look for said combos within a given vector
        pub fn two_addends() -> f32 {
            let x: f32 = 5 as f32;
            let y: f32 = x / 2.0;
            let z: i32 = y.ceil() as i32;
            let a: i32 = y.floor() as i32;

            println!("z: {}, a: {}", z, a);
            y
            /*
            let y: i32 = x.ceil() as i32;
            let z: i32 = x.floor() as i32;
            println!("y: {}, z: {}", y, z);
            y
            */
            /*
            let mut first: i32 = 0;
            let mut second: i32 = 0;
            let mut i: i32 = 0;
            loop {
                if i < num {
                    println!("in here");
                    i += 1;
                    continue;
                }
                break;
            }
            vec![1, 2]
            */
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
