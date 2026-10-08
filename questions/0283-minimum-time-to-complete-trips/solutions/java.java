class Solution {
    public long minimumTime(int[] time, int totalTrips) {
        int fastest = Integer.MAX_VALUE;
        for (int x : time) fastest = Math.min(fastest, x);
        long lo = 1, hi = (long) fastest * totalTrips;
        while (lo < hi) {
            long mid = lo + (hi - lo) / 2;
            if (enough(time, totalTrips, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    // Stop as soon as the target is reached so the running count cannot overflow.
    private boolean enough(int[] time, int totalTrips, long t) {
        long done = 0;
        for (int x : time) {
            done += t / x;
            if (done >= totalTrips) return true;
        }
        return false;
    }
}
