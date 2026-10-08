class Solution {
    public long maxMinPower(int[] stations, int r, int k) {
        int n = stations.length;
        // power[i] = sum of stations in [i - r, i + r], via a sliding window.
        long[] power = new long[n];
        long window = 0;
        for (int i = 0; i < Math.min(n, r + 1); i++) window += stations[i];
        for (int i = 0; i < n; i++) {
            power[i] = window;
            if (i + r + 1 < n) window += stations[i + r + 1];
            if (i - r >= 0) window -= stations[i - r];
        }
        long lo = Long.MAX_VALUE;
        for (long p : power) lo = Math.min(lo, p);
        long hi = lo + k;
        long[] added = new long[n + 1];
        while (lo < hi) {
            long mid = lo + (hi - lo + 1) / 2;
            if (feasible(power, added, r, k, mid)) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }

    private boolean feasible(long[] power, long[] added, int r, long k, long target) {
        int n = power.length;
        Arrays.fill(added, 0);
        long extra = 0, used = 0;
        for (int i = 0; i < n; i++) {
            extra += added[i];
            long have = power[i] + extra;
            if (have < target) {
                long need = target - have;
                used += need;
                if (used > k) return false;
                extra += need;
                added[(int) Math.min(n, (long) i + 2L * r + 1)] -= need;
            }
        }
        return true;
    }
}
