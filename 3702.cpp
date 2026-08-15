class Solution {
public:
    int longestSubsequence(vector<int>& nums) {
        int total_xor = 0, n = nums.size();
        for (int x : nums) {
            total_xor ^= x;
        }
        if (total_xor != 0) {
            return n;
        }
        for (int x : nums) {
            if (x != 0) {
                return n - 1;
            }
        }
        return 0;
    }
};
