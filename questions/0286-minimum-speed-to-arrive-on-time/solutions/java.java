class Solution {
    private boolean onTime(int[] dist, long total, int speed) {
        long whole = 0;
        for (int i = 0; i < dist.length - 1; i++) whole += (dist[i] + speed - 1) / speed;
        long rest = total - whole * 100;
        long last = dist[dist.length - 1] * 100L;
        return rest >= 0 && (rest >= last || last <= rest * speed);
    }

    public int minSpeedOnTime(int[] dist, double hour) {
        long total = Math.round(hour * 100);
        int lo = 1, hi = 10_000_000;
        if (!onTime(dist, total, hi)) return -1;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (onTime(dist, total, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }
}
