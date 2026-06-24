#include <vector>

class Solution {
private:
    long long MOD = 1e9 + 7;

    std::vector<std::vector<long long>> multiply(const std::vector<std::vector<long long>>& A, 
                                                 const std::vector<std::vector<long long>>& B) {
        int sz = A.size();
        std::vector<std::vector<long long>> C(sz, std::vector<long long>(sz, 0));
        for (int i = 0; i < sz; ++i) {
            for (int k = 0; k < sz; ++k) {
                if (A[i][k] == 0) continue;
                for (int j = 0; j < sz; ++j) {
                    C[i][j] = (C[i][j] + A[i][k] * B[k][j]) % MOD;
                }
            }
        }
        return C;
    }

    std::vector<std::vector<long long>> power(std::vector<std::vector<long long>> A, long long p) {
        int sz = A.size();
        std::vector<std::vector<long long>> res(sz, std::vector<long long>(sz, 0));
        for (int i = 0; i < sz; ++i) res[i][i] = 1;

        while (p > 0) {
            if (p & 1) res = multiply(res, A);
            A = multiply(A, A);
            p >>= 1;
        }
        return res;
    }

public:
    int zigZagArrays(int n, int l, int r) {
        int m = r - l + 1;
        if (n == 1) return m;

        int sz = 2 * m;
        std::vector<std::vector<long long>> T(sz, std::vector<long long>(sz, 0));

        for (int x = 0; x < m; ++x) {
            for (int y = x + 1; y < m; ++y) {
                T[m + y][x] = 1; 
            }
            for (int y = 0; y < x; ++y) {
                T[y][m + x] = 1;
            }
        }

        T = power(T, n - 1);

        long long total_arrays = 0;
        for (int i = 0; i < sz; ++i) {
            long long ways_for_state = 0;
            for (int j = 0; j < sz; ++j) {
                ways_for_state = (ways_for_state + T[i][j]) % MOD;
            }
            total_arrays = (total_arrays + ways_for_state) % MOD;
        }

        return total_arrays;
    }
};
