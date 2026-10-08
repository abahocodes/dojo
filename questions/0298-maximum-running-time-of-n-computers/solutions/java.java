class Solution {
    public long maxRunTime(int n, int[] batteries) {
        long total = 0;
        for (int b : batteries) total += b;
        long lo = 0, hi = total / n;
        while (lo < hi) {
            long mid = lo + (hi - lo + 1) / 2;
            if (canRun(n, batteries, mid)) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }

    private boolean canRun(int n, int[] batteries, long minutes) {
        long usable = 0;
        for (int b : batteries) usable += Math.min((long) b, minutes);
        return usable >= (long) n * minutes;
    }
}
