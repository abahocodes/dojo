class Solution {
public:
    string longestPalindrome(string& s) {
        int n = s.size();
        int bestLo = 0, bestLen = 1;
        for (int center = 0; center < n; center++) {
            for (int odd = 0; odd < 2; odd++) {
                int lo = center, hi = center + odd;
                while (lo >= 0 && hi < n && s[lo] == s[hi]) {
                    lo--;
                    hi++;
                }
                int length = hi - lo - 1;
                if (length > bestLen) {
                    bestLo = lo + 1;
                    bestLen = length;
                }
            }
        }
        return s.substr(bestLo, bestLen);
    }
};
