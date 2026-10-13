impl Solution {
    pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
        let mut squared_nums: Vec<i64> = nums
            .into_iter()
            .map(|x| i64::from(x) * i64::from(x))
            .collect();
        squared_nums.sort();

        let m = squared_nums.len() >> 1;
        let mut max_score = 0i64;

        for (i, &x) in squared_nums.iter().enumerate() {
            if i < m {
                max_score -= x;
            } else {
                max_score += x;
            }
        }

        max_score
    }
}
