impl Solution {
    pub fn min_sum_square_diff(nums1: Vec<i32>, nums2: Vec<i32>, k1: i32, k2: i32) -> i64 {
        let mut k = k1 as i64 + k2 as i64;
        let mut max_diff = 0;

        let mut counts = vec![0i64; 100001];
        
        for i in 0..nums1.len() {
            let diff = (nums1[i] - nums2[i]).abs() as usize;
            counts[diff] += 1;
            if diff > max_diff {
                max_diff = diff;
            }
        }

        for d in (1..=max_diff).rev() {
            if counts[d] == 0 {
                continue;
            }
            
            let needed = counts[d];
            
            if k >= needed {
                k -= needed;
                counts[d - 1] += needed;
                counts[d] = 0;
            } else {
                counts[d - 1] += k;
                counts[d] -= k;
                k = 0;
                break;
            }
        }
        let mut ans: i64 = 0;
        for d in 1..=max_diff {
            if counts[d] > 0 {
                ans += (d as i64) * (d as i64) * counts[d];
            }
        }
        
        ans
    }
}
