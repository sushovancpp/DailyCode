class Solution {
public:
    int sumCounts(std::vector<int>& nums) {
        int n = nums.size();
        int totalSum = 0;

        for (int i = 0; i < n; ++i) {
            std::unordered_set<int> distinct;
            for (int j = i; j < n; ++j) {
                distinct.insert(nums[j]);
                int count = distinct.size();
                totalSum += count * count;
            }
        }

        return totalSum;
    }
};
