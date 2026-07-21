class Solution {
public:
    int maxActiveSectionsAfterTrade(std::string s) {
        int n = s.length();
        int totalOnes = 0;           
        int currentIndex = 0;        
        int previousZeroSegment = INT_MIN;  
        int maxZeroWindow = 0;      
        while (currentIndex < n) {

            int segmentEnd = currentIndex + 1;
            while (segmentEnd < n && s[segmentEnd] == s[currentIndex]) {
                segmentEnd++;
            }
          

            int segmentLength = segmentEnd - currentIndex;
          
            if (s[currentIndex] == '1') {
                totalOnes += segmentLength;
            } else {

                maxZeroWindow = std::max(maxZeroWindow, previousZeroSegment + segmentLength);
                previousZeroSegment = segmentLength;
            }
          
            currentIndex = segmentEnd;
        }
    
        return totalOnes + maxZeroWindow;
    }
};
