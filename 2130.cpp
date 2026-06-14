class Solution {
public:
    int pairSum(ListNode* head) {
        vector<int> s;
        for (; head != nullptr; head = head->next) s.push_back(head->val);
        int ans = 0, n = s.size();
        for (int i = 0; i < (n >> 1); ++i) ans = max(ans, s[i] + s[n - i - 1]);
        return ans;
    }
};
