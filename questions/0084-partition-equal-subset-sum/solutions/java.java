class Solution {
    public boolean canPartition(int[] nums) {
        int total = 0;
        for (int x : nums) total += x;
        if (total % 2 != 0) return false;
        int half = total / 2;
        // bit s of reachable is set when some subset weighs exactly s
        long[] reachable = new long[half / 64 + 1];
        reachable[0] = 1L;
        for (int x : nums) {
            // reachable |= reachable << x, word by word from the top down
            int q = x / 64, r = x % 64;
            for (int i = reachable.length - 1; i >= q; i--) {
                long shifted = reachable[i - q] << r;
                if (r > 0 && i - q - 1 >= 0) shifted |= reachable[i - q - 1] >>> (64 - r);
                reachable[i] |= shifted;
            }
        }
        return ((reachable[half / 64] >>> (half % 64)) & 1L) == 1L;
    }
}
