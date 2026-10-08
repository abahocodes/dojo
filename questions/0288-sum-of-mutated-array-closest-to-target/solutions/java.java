class Solution {
    private long capped(int[] arr, int v) {
        long s = 0;
        for (int a : arr) s += Math.min(a, v);
        return s;
    }

    public int findBestValue(int[] arr, int target) {
        int lo = 0, hi = 0;
        for (int a : arr) hi = Math.max(hi, a);
        if (capped(arr, hi) < target) return hi;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (capped(arr, mid) >= target) hi = mid;
            else lo = mid + 1;
        }
        if (lo > 0 && target - capped(arr, lo - 1) <= capped(arr, lo) - target) return lo - 1;
        return lo;
    }
}
