class Solution {
    public int longestAwesome(String s) {
        int n = s.length();
        int[] first = new int[1024];
        Arrays.fill(first, n + 1);
        first[0] = 0;
        int mask = 0, best = 0;
        for (int i = 1; i <= n; i++) {
            mask ^= 1 << (s.charAt(i - 1) - '0');
            best = Math.max(best, i - first[mask]);
            for (int d = 0; d < 10; d++) {
                best = Math.max(best, i - first[mask ^ (1 << d)]);
            }
            if (first[mask] > i) first[mask] = i;
        }
        return best;
    }
}
