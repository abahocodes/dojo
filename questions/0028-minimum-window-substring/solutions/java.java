class Solution {
    public String minWindow(String s, String t) {
        Map<Character, Integer> need = new HashMap<>();
        for (int i = 0; i < t.length(); i++) need.merge(t.charAt(i), 1, Integer::sum);
        int missing = t.length();
        int bestStart = 0, bestLen = s.length() + 1;
        int left = 0;
        for (int right = 0; right < s.length(); right++) {
            char ch = s.charAt(right);
            int have = need.getOrDefault(ch, 0);
            if (have > 0) missing--;
            need.put(ch, have - 1);
            while (missing == 0) {
                if (right - left + 1 < bestLen) {
                    bestStart = left;
                    bestLen = right - left + 1;
                }
                char out = s.charAt(left);
                int next = need.get(out) + 1;
                need.put(out, next);
                if (next > 0) missing++;
                left++;
            }
        }
        if (bestLen > s.length()) return "";
        return s.substring(bestStart, bestStart + bestLen);
    }
}
