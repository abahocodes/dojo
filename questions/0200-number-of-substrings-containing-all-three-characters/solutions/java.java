class Solution {
    public int numberOfSubstrings(String s) {
        int[] last = {-1, -1, -1};
        int total = 0;
        for (int i = 0; i < s.length(); i++) {
            last[s.charAt(i) - 'a'] = i;
            total += Math.min(last[0], Math.min(last[1], last[2])) + 1;
        }
        return total;
    }
}
