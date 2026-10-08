class Solution {
    public double minmaxGasDist(int[] stations, int k) {
        double lo = 0, hi = 0;
        for (int i = 1; i < stations.length; i++) {
            hi = Math.max(hi, stations[i] - stations[i - 1]);
        }
        for (int iter = 0; iter < 100; iter++) {
            double mid = (lo + hi) / 2;
            if (fits(stations, k, mid)) hi = mid;
            else lo = mid;
        }
        return hi;
    }

    private boolean fits(int[] stations, int k, double limit) {
        long added = 0;
        for (int i = 1; i < stations.length; i++) {
            added += (long) ((stations[i] - stations[i - 1]) / limit);
            if (added > k) return false;
        }
        return true;
    }
}
