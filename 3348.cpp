#include <string>
#include <vector>
#include <algorithm>
#include <numeric>

using namespace std;

class Solution {
    struct Primes {
        int c2 = 0, c3 = 0, c5 = 0, c7 = 0;
        bool operator==(const Primes& o) const {
            return c2 == o.c2 && c3 == o.c3 && c5 == o.c5 && c7 == o.c7;
        }
    };

    Primes getPrimes(long long t) {
        Primes p;
        while (t % 2 == 0) { p.c2++; t /= 2; }
        while (t % 3 == 0) { p.c3++; t /= 3; }
        while (t % 5 == 0) { p.c5++; t /= 5; }
        while (t % 7 == 0) { p.c7++; t /= 7; }
        return p;
    }

    const Primes digitPrimes[10] = {
        {0, 0, 0, 0}, {0, 0, 0, 0}, {1, 0, 0, 0}, {0, 1, 0, 0},
        {2, 0, 0, 0}, {0, 0, 1, 0}, {1, 1, 0, 0}, {0, 0, 0, 1},
        {3, 0, 0, 0}, {0, 2, 0, 0}
    };

    Primes add(Primes a, Primes b) {
        return {a.c2 + b.c2, a.c3 + b.c3, a.c5 + b.c5, a.c7 + b.c7};
    }

    Primes sub(Primes a, Primes b) {
        return {max(0, a.c2 - b.c2), max(0, a.c3 - b.c3), max(0, a.c5 - b.c5), max(0, a.c7 - b.c7)};
    }

    bool satisfies(const Primes& cur, const Primes& req) {
        return cur.c2 >= req.c2 && cur.c3 >= req.c3 && cur.c5 >= req.c5 && cur.c7 >= req.c7;
    }

    int totalCount(Primes p) {
        int count8 = p.c2 / 3;
        int rem2 = p.c2 % 3;
        int count9 = p.c3 / 2;
        int rem3 = p.c3 % 2;
        int count4 = rem2 / 2;
        int count2 = rem2 % 2;
        int count6 = 0;
        if (count2 == 1 && rem3 == 1) { count2 = 0; rem3 = 0; count6 = 1; }
        if (rem3 == 1 && count4 == 1) { count2 = 1; count6 = 1; count4 = 0; rem3 = 0; }
        return count8 + count9 + count4 + p.c5 + count6 + p.c7 + count2 + rem3;
    }

    string construct(Primes p) {
        int count8 = p.c2 / 3;
        int rem2 = p.c2 % 3;
        int count9 = p.c3 / 2;
        int rem3 = p.c3 % 2;
        int count4 = rem2 / 2;
        int count2 = rem2 % 2;
        int count6 = 0;
        if (count2 == 1 && rem3 == 1) { count2 = 0; rem3 = 0; count6 = 1; }
        if (rem3 == 1 && count4 == 1) { count2 = 1; count6 = 1; count4 = 0; rem3 = 0; }

        string res = "";
        res.append(count2, '2');
        res.append(rem3, '3');
        res.append(count4, '4');
        res.append(p.c5, '5');
        res.append(count6, '6');
        res.append(p.c7, '7');
        res.append(count8, '8');
        res.append(count9, '9');
        return res;
    }

public:
    string smallestNumber(string num, long long t) {
        Primes req = getPrimes(t);
        long long rem = t;
        for (int p : {2, 3, 5, 7}) {
            while (rem % p == 0) rem /= p;
        }
        if (rem > 1) return "-1";

        Primes reqDigitsCost = getPrimes(t);
        int minLenNeeded = totalCount(reqDigitsCost);
        if (minLenNeeded > (int)num.length()) {
            return construct(reqDigitsCost);
        }

        int firstZero = num.find('0');
        if (firstZero == -1) firstZero = num.length();

        Primes prefixPrimes;
        for (int i = 0; i < firstZero; ++i) {
            prefixPrimes = add(prefixPrimes, digitPrimes[num[i] - '0']);
        }

        if (firstZero == num.length() && satisfies(prefixPrimes, req)) {
            return num;
        }

        for (int i = num.length() - 1; i >= 0; --i) {
            if (i < firstZero) {
                prefixPrimes = sub(prefixPrimes, digitPrimes[num[i] - '0']);
            }
            int d = num[i] - '0';
            int space = num.length() - 1 - i;

            if (i > firstZero) continue;

            for (int nd = d + 1; nd <= 9; ++nd) {
                Primes neededAfter = sub(req, add(prefixPrimes, digitPrimes[nd]));
                if (totalCount(neededAfter) <= space) {
                    int ones = space - totalCount(neededAfter);
                    return num.substr(0, i) + to_string(nd) + string(ones, '1') + construct(neededAfter);
                }
            }
        }

        return string(num.length() + 1 - minLenNeeded, '1') + construct(reqDigitsCost);
    }
};
