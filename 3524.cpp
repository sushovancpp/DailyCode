class Solution {
 public:
  vector<long long> resultArray(vector<int>& nums, int k) {
    vector<long long> ans(k);
    vector<long long> dp(k);

    for (const int num : nums) {
      vector<long long> newDp(k);
      const int numMod = num % k;
      newDp[numMod] = 1;
      for (int i = 0; i < k; ++i) {
        const int newMod = (static_cast<long>(i) * numMod) % k;
        newDp[newMod] += dp[i];
      }
      for (int i = 0; i < k; ++i)
        ans[i] += newDp[i];
      dp = std::move(newDp);
    }

    return ans;
  }
};
