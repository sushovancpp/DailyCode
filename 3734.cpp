class Solution {
public:
    string buildPalindrome(const string& half, char mid) {
        string res = half;

        if (mid != '#')
            res += mid;

        string rev = half;
        reverse(rev.begin(), rev.end());

        res += rev;
        return res;
    }

    string lexPalindromicPermutation(string s, string target) {
        int n = s.size();
        int m = n / 2;

        vector<int> cnt(26, 0);

        for (char c : s)
            cnt[c - 'a']++;

        // More than one odd-frequency character
        // means no palindrome is possible.
        int odd = 0;
        char mid = '#';

        for (int i = 0; i < 26; i++) {
            if (cnt[i] % 2) {
                odd++;
                mid = char('a' + i);
            }
        }

        if (odd > 1)
            return "";

        // Characters available for the left half.
        vector<int> halfCnt(26);

        for (int i = 0; i < 26; i++)
            halfCnt[i] = cnt[i] / 2;

        string targetHalf = target.substr(0, m);

        // -------------------------------------------------
        // Case 1: targetHalf itself can be formed.
        // -------------------------------------------------
        vector<int> rem = halfCnt;
        bool possible = true;

        for (char c : targetHalf) {
            int x = c - 'a';

            if (rem[x] == 0) {
                possible = false;
                break;
            }

            rem[x]--;
        }

        if (possible) {
            string pal = buildPalindrome(targetHalf, mid);

            if (pal > target)
                return pal;
        }

        // -------------------------------------------------
        // Case 2: Find the smallest half > targetHalf.
        // -------------------------------------------------
        for (int k = m - 1; k >= 0; k--) {

            rem = halfCnt;
            possible = true;

            // Match targetHalf[0 ... k-1]
            for (int i = 0; i < k; i++) {
                int x = targetHalf[i] - 'a';

                if (rem[x] == 0) {
                    possible = false;
                    break;
                }

                rem[x]--;
            }

            if (!possible)
                continue;

            int cur = targetHalf[k] - 'a';

            // Pick the smallest character greater than target[k].
            for (int x = cur + 1; x < 26; x++) {

                if (rem[x] == 0)
                    continue;

                rem[x]--;

                string half = targetHalf.substr(0, k);
                half += char('a' + x);

                // Fill remaining characters in sorted order.
                for (int c = 0; c < 26; c++) {
                    half += string(rem[c], char('a' + c));
                }

                return buildPalindrome(half, mid);
            }
        }

        return "";
    }
};