class Solution {
    public int threeSumMulti(int[] arr, int target) {
        final long MOD = 1_000_000_007L;
        long[] c = new long[101];
        for (int v : arr) c[v]++;
        long total = 0;
        for (int x = 0; x <= 100; x++) {
            for (int y = x; y <= 100; y++) {
                int z = target - x - y;
                if (z < y || z > 100) continue;
                long ways;
                if (x == y && y == z) ways = c[x] * (c[x] - 1) * (c[x] - 2) / 6;
                else if (x == y) ways = c[x] * (c[x] - 1) / 2 * c[z];
                else if (y == z) ways = c[x] * (c[y] * (c[y] - 1) / 2);
                else ways = c[x] * c[y] * c[z];
                total = (total + ways % MOD) % MOD;
            }
        }
        return (int) total;
    }
}
