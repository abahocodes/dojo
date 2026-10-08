class Solution {
    public long wonderfulSubstrings(String word) {
        long[] seen = new long[1024];
        seen[0] = 1;
        int mask = 0;
        long total = 0;
        for (int i = 0; i < word.length(); i++) {
            mask ^= 1 << (word.charAt(i) - 'a');
            total += seen[mask];
            for (int k = 0; k < 10; k++) total += seen[mask ^ (1 << k)];
            seen[mask]++;
        }
        return total;
    }
}
