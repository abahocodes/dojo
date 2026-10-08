class Solution {
    public int longestSubstringKRepeating(String s, int k) {
        int best = 0;
        for (int limit = 1; limit <= 26; limit++) {
            int[] count = new int[26];
            int left = 0, unique = 0, atLeast = 0;
            for (int right = 0; right < s.length(); right++) {
                int c = s.charAt(right) - 'a';
                if (count[c] == 0) unique++;
                count[c]++;
                if (count[c] == k) atLeast++;
                while (unique > limit) {
                    int d = s.charAt(left) - 'a';
                    if (count[d] == k) atLeast--;
                    count[d]--;
                    if (count[d] == 0) unique--;
                    left++;
                }
                if (unique == atLeast) best = Math.max(best, right - left + 1);
            }
        }
        return best;
    }
}
