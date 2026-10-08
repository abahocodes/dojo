class Solution {
    public int splitArray(int[] nums, int k) {
        long lo = 0, hi = 0;
        for (int x : nums) {
            lo = Math.max(lo, x);
            hi += x;
        }
        while (lo < hi) {
            long mid = lo + (hi - lo) / 2;
            if (piecesNeeded(nums, mid) <= k) hi = mid;
            else lo = mid + 1;
        }
        return (int) lo;
    }

    private int piecesNeeded(int[] nums, long cap) {
        int pieces = 1;
        long current = 0;
        for (int x : nums) {
            if (current + x > cap) {
                pieces++;
                current = x;
            } else {
                current += x;
            }
        }
        return pieces;
    }
}
