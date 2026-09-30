class Solution {
 public:
  vector<int> maxDepthAfterSplit(string seq) {
    vector<int> ans;
    int depth = 1;

    for (const char c : seq)
      if (c == '(')
        ans.push_back(++depth % 2);
      else
        ans.push_back(depth-- % 2);

    return ans;
  }
};
