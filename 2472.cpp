class Solution {
 public:
  int maxPalindromes(string s, int k) {
    const int n = s.length();
    vector<int> dp(n + 1);
    for (int i = k; i <= n; ++i) {
      dp[i] = dp[i - 1];
      if (isPalindrome(s, i - k, i - 1))
        dp[i] = max(dp[i], 1 + dp[i - k]);
      if (isPalindrome(s, i - k - 1, i - 1))
        dp[i] = max(dp[i], 1 + dp[i - k - 1]);
    }

    return dp[n];
  }

 private:
  bool isPalindrome(const string& s, int l, int r) {
    if (l < 0)
      return false;
    while (l < r)
      if (s[l++] != s[r--])
        return false;
    return true;
  }
};
