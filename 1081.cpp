
class Solution {
public:
    string smallestSubsequence(string s) {
        int stringLength = s.size();

        int lastOccurrence[26] = {0};
        for (int i = 0; i < stringLength; ++i) {
            lastOccurrence[s[i] - 'a'] = i;
        }

        string result;
        int usedCharMask = 0;

        for (int i = 0; i < stringLength; ++i) {
            char currentChar = s[i];
            int charIndex = currentChar - 'a';

            if ((usedCharMask >> charIndex) & 1) {
                continue;
            }
          
            while (!result.empty() && 
                   result.back() > currentChar && 
                   lastOccurrence[result.back() - 'a'] > i) {
                usedCharMask ^= 1 << (result.back() - 'a');
                result.pop_back();
            }
            result.push_back(currentChar);
            usedCharMask |= 1 << charIndex;
        }
      
        return result;
    }
};