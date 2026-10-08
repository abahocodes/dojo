class Solution {
    public String customSortString(String order, String s) {
        int[] counts = new int[26];
        for (int i = 0; i < s.length(); i++) counts[s.charAt(i) - 'a']++;
        boolean[] ranked = new boolean[26];
        StringBuilder sb = new StringBuilder(s.length());
        for (int i = 0; i < order.length(); i++) {
            char c = order.charAt(i);
            ranked[c - 'a'] = true;
            for (int k = 0; k < counts[c - 'a']; k++) sb.append(c);
        }
        for (int i = 0; i < s.length(); i++) {
            char c = s.charAt(i);
            if (!ranked[c - 'a']) sb.append(c);
        }
        return sb.toString();
    }
}
