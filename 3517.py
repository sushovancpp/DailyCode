class Solution:
    def smallestPalindrome(self, s: str) -> str:

        count = [0] * 26
        for char in s:
            count[ord(char) - ord('a')] += 1
            
        first_half = []
        mid_char = ""
        

        for i in range(26):
            if count[i] > 0:
                char = chr(ord('a') + i)
                first_half.append(char * (count[i] // 2))
                if count[i] % 2 == 1:
                    mid_char = char
                    
        left_str = "".join(first_half)
        
        return left_str + mid_char + left_str[::-1]
