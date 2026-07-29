#include <bits/stdc++.h>
using namespace std;

class Solution {
    long long MAX_K = 1000000LL;

    long long countPermutations(vector<int>& count, int total) {
        long long res = 1;
        int remaining = total;
        for (int c : count) {
            if (c == 0) continue;
            for (int j = 1; j <= c; ++j) {
                res = res * (remaining - c + j) / j;
                if (res > MAX_K) return MAX_K + 1;
            }
            remaining -= c;
        }
        return res;
    }

public:
    string smallestPalindrome(string s, long long k) {
        vector<int> freq(26, 0);
        for (char c : s) {
            freq[c - 'a']++;
        }

        vector<int> halfCount(26, 0);
        string midChar = "";
        int halfLen = 0;

        for (int i = 0; i < 26; ++i) {
            halfCount[i] = freq[i] / 2;
            halfLen += halfCount[i];
            if (freq[i] % 2 != 0) {
                midChar = string(1, i + 'a');
            }
        }

        long long totalPerms = countPermutations(halfCount, halfLen);
        if (k > totalPerms) {
            return "";
        }

        string leftHalf = "";
        int currentLen = halfLen;

        for (int i = 0; i < halfLen; ++i) {
            for (int j = 0; j < 26; ++j) {
                if (halfCount[j] == 0) continue;
                
                halfCount[j]--;
                currentLen--;
                long long perms = countPermutations(halfCount, currentLen);
                
                if (k <= perms) {
                    leftHalf += (char)(j + 'a');
                    break;
                } else {
                    k -= perms;
                    halfCount[j]++;
                    currentLen++;
                }
            }
        }

        string rightHalf = leftHalf;
        reverse(rightHalf.begin(), rightHalf.end());

        return leftHalf + midChar + rightHalf;
    }
};
