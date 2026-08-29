class Solution {
 public:
  vector<int> lexicographicallySmallestArray(vector<int>& nums, int limit) {
    vector<int> ans(nums.size());

    // Groups of (number, original index).
    // Within each group, consecutive numbers differ by <= limit.
    vector<vector<pair<int, int>>> numAndIndexesGroups;

    for (const auto& [num, index] : getNumAndIndexes(nums)) {
      if (numAndIndexesGroups.empty() ||
          num - numAndIndexesGroups.back().back().first > limit) {
        // Start a new group.
        numAndIndexesGroups.push_back({{num, index}});
      } else {
        // Add to the current group.
        numAndIndexesGroups.back().push_back({num, index});
      }
    }

    for (const auto& group : numAndIndexesGroups) {
      vector<int> sortedNums;
      vector<int> sortedIndices;

      for (const auto& [num, index] : group) {
        sortedNums.push_back(num);
        sortedIndices.push_back(index);
      }

      // Put the smallest values at the smallest indices.
      ranges::sort(sortedIndices);

      for (int i = 0; i < sortedNums.size(); ++i)
        ans[sortedIndices[i]] = sortedNums[i];
    }

    return ans;
  }

 private:
  vector<pair<int, int>> getNumAndIndexes(const vector<int>& nums) {
    vector<pair<int, int>> numAndIndexes;

    for (int i = 0; i < nums.size(); ++i)
      numAndIndexes.emplace_back(nums[i], i);

    // Sort by number, then by index.
    ranges::sort(numAndIndexes);

    return numAndIndexes;
  }
};