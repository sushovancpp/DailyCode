#include <vector>

class Solution {
public:
    int zigZagArrays(int n, int l, int r) {
        int m = r - l + 1;
        long long MOD = 1e9 + 7;
        
        std::vector<std::vector<long long>> dp(2, std::vector<long long>(m, 1));
        std::vector<std::vector<long long>> next_dp(2, std::vector<long long>(m, 0));
        
        for (int i = 2; i <= n; ++i) {
            long long prefix_sum = 0;
            for (int y = 0; y < m; ++y) {
                next_dp[0][y] = prefix_sum;
                prefix_sum = (prefix_sum + dp[1][y]) % MOD;
            }
            
            long long suffix_sum = 0;
            for (int y = m - 1; y >= 0; --y) {
                next_dp[1][y] = suffix_sum;
                suffix_sum = (suffix_sum + dp[0][y]) % MOD;
            }
            
            dp = next_dp;
        }
        
        long long total_valid_arrays = 0;
        for (int x = 0; x < m; ++x) {
            total_valid_arrays = (total_valid_arrays + dp[0][x] + dp[1][x]) % MOD;
        }
        
        return total_valid_arrays;
    }
};
