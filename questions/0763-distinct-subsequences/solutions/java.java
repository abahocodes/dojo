class Solution {
    public int numDistinct(String s, String t) {
        int m = t.length();
        // ways[j] = number of ways to form t[:j] from the part of s read so far.
        // Counts for short prefixes of t can overflow; long arithmetic wraps
        // modulo 2^64, and since the true answer fits in an int, ways[m] is exact.
        long[] ways = new long[m + 1];
        ways[0] = 1;
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            // Go right to left so ways[j - 1] still holds the previous row's value.
            for (int j = m; j >= 1; j--) {
                if (t.charAt(j - 1) == ch) ways[j] += ways[j - 1];
            }
        }
        return (int) ways[m];
    }
}
