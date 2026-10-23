class Solution {
public:
    int sortPermutation(vector<int>& nums) {
        int result = -1;
        for (int i = 0; i < nums.size(); ++i) {
            if (i != nums[i]) {
                result &= nums[i];
            }
        }
        return max(result, 0);
    }
};
