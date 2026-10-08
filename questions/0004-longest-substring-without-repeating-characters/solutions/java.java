class Solution {
    public int lengthOfLongestSubstring(String s) {
        Map<Character, Integer> last = new HashMap<>();
        int left = 0, best = 0;
        for (int i = 0; i < s.length(); i++) {
            Integer prev = last.get(s.charAt(i));
            if (prev != null && prev >= left) left = prev + 1;
            last.put(s.charAt(i), i);
            best = Math.max(best, i - left + 1);
        }
        return best;
    }
}
