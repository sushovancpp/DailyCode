class Solution {
public:
    static constexpr int MOD = 1e9 + 7;
    int n;
    vector<int> nums;
    int dp[201][201][201];

    int solve(int i, int g1, int g2) {
        if (i == n)
            return (g1 == g2 && g1 != 0);

        int &res = dp[i][g1][g2];
        if (res != -1) return res;

        res = solve(i + 1, g1, g2); 

        int ng1 = (g1 == 0 ? nums[i] : gcd(g1, nums[i]));
        res = (res + solve(i + 1, ng1, g2)) % MOD;

        int ng2 = (g2 == 0 ? nums[i] : gcd(g2, nums[i]));
        res = (res + solve(i + 1, g1, ng2)) % MOD;

        return res;
    }

    int subsequencePairCount(vector<int>& nums) {
        this->nums = nums;
        n = nums.size();
        memset(dp, -1, sizeof(dp));
        return solve(0, 0, 0);
    }
};