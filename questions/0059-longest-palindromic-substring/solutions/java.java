class Solution {
    public String longestPalindrome(String s) {
        int n = s.length();
        int bestLo = 0, bestLen = 1;
        for (int center = 0; center < n; center++) {
            for (int odd = 0; odd < 2; odd++) {
                int lo = center, hi = center + odd;
                while (lo >= 0 && hi < n && s.charAt(lo) == s.charAt(hi)) {
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
        return s.substring(bestLo, bestLo + bestLen);
    }
}
