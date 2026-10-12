impl Solution {
    pub fn max_sum(nums: Vec<i32>, k: i32) -> i32 {
        const MOD: i64 = 1_000_000_007;
        // Since nums[i] <= 10^9, 30 bits are sufficient (2^30 > 10^9)
        let mut bit_counts = vec![0; 30];
        
        // Step 1: Count the frequency of '1' bits at each position
        for &num in &nums {
            for i in 0..30 {
                if (num >> i) & 1 == 1 {
                    bit_counts[i] += 1;
                }
            }
        }
        
        let mut total_sum_of_squares: i64 = 0;
        
        // Step 2: Greedily build the k largest possible numbers
        for _ in 0..k {
            let mut current_num: i64 = 0;
            for i in 0..30 {
                if bit_counts[i] > 0 {
                    current_num |= 1 << i;
                    bit_counts[i] -= 1;
                }
            }
            // Add the square of the constructed number modulo 10^9 + 7
            let square = (current_num * current_num) % MOD;
            total_sum_of_squares = (total_sum_of_squares + square) % MOD;
        }
        
        total_sum_of_squares as i32
    }
}
