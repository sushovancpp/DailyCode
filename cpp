class Solution {
 public:
  int stoneGameVIII(vector<int>& stones) {
    const int n = stones.size();
    vector<int> prefix(n);
    vector<int> dp(n, INT_MIN);

    partial_sum(stones.begin(), stones.end(), prefix.begin());

    dp[n - 2] = prefix.back();

    for (int i = n - 3; i >= 0; --i)
      dp[i] = max(dp[i + 1], prefix[i + 1] - dp[i + 1]);

    return dp[0];
  }
}
