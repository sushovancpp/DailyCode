#include <vector>

using namespace std;

class Solution {
public:
    int uniquePaths(vector<vector<int>>& grid) {
        int m = grid.size();
        if (m == 0) return 0;
        int n = grid[0].size();
        const int MOD = 1e9 + 7;

        vector<vector<long long>> dp(n + 1, vector<long long>(2, 0));
        dp[1][0] = 1;
        dp[1][1] = 1;

        for (int r = 0; r < m; ++r) {
            for (int c = 0; c < n; ++c) {
                if (grid[r][c] == 1) {
                    long long next_0 = dp[c + 1][1];
                    long long next_1 = dp[c][0];
                    dp[c + 1][0] = next_0;
                    dp[c + 1][1] = next_1;
                } else {
                    long long sum = (dp[c + 1][1] + dp[c][0]) % MOD;
                    dp[c + 1][0] = sum;
                    dp[c + 1][1] = sum;
                }
            }
        }

        return dp[n][0];
    }
};
