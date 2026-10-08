class Solution {
    public int lengthOfLongestSubstringKDistinct(String s, int k) {
        int[] count = new int[128];
        int distinct = 0, left = 0, best = 0;
        for (int right = 0; right < s.length(); right++) {
            if (count[s.charAt(right)]++ == 0) distinct++;
            while (distinct > k) {
                if (--count[s.charAt(left)] == 0) distinct--;
                left++;
            }
            best = Math.max(best, right - left + 1);
        }
        return best;
    }
}
