class Solution {
public:
    vector<int> gcdValues(vector<int>& nums, vector<long long>& queries) {

        int maxValue = ranges::max(nums);


        vector<int> frequencyCount(maxValue + 1);
        for (int number : nums) {
            ++frequencyCount[number];
        }


        vector<long long> gcdPairCount(maxValue + 1);

 
        for (int gcd = maxValue; gcd >= 1; --gcd) {

            long long multiplesCount = 0;

            for (int multiple = gcd; multiple <= maxValue; multiple += gcd) {
                multiplesCount += frequencyCount[multiple];

                gcdPairCount[gcd] -= gcdPairCount[multiple];
            }

            gcdPairCount[gcd] += multiplesCount * (multiplesCount - 1) / 2;
        }

        for (int i = 2; i <= maxValue; ++i) {
            gcdPairCount[i] += gcdPairCount[i - 1];
        }

        vector<int> result;
        for (const auto& query : queries) {

            int gcdValue = upper_bound(gcdPairCount.begin(), gcdPairCount.end(), query) - gcdPairCount.begin();
            result.push_back(gcdValue);
        }

        return result;
    }
};
