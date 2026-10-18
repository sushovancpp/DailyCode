#include <iostream>
#include <vector>

using namespace std;

class Solution {
    long long MOD = 1e9 + 7;
    vector<vector<long long>> table;

    void generatePascal(int n) {
        table.assign(n + 1, vector<long long>(n + 1, 1));
        for (int i = 2; i <= n; i++) {
            for (int j = 1; j < i; j++) {
                table[i][j] = (table[i - 1][j - 1] + table[i - 1][j]) % MOD;
            }
        }
    }

    long long dfs(vector<int>& nums) {
        if (nums.size() <= 2) return 1;

        vector<int> left, right;
        int root = nums[0];

        for (int i = 1; i < nums.size(); i++) {
            if (nums[i] < root) {
                left.push_back(nums[i]);
            } else {
                right.push_back(nums[i]);
            }
        }

        long long leftWays = dfs(left);
        long long rightWays = dfs(right);

        return (((leftWays * rightWays) % MOD) * table[left.size() + right.size()][left.size()]) % MOD;
    }

public:
    int numOfWays(vector<int>& nums) {
        int n = nums.size();
        generatePascal(n);
        return (dfs(nums) - 1 + MOD) % MOD;
    }
};
