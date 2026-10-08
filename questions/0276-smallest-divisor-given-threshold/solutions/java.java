class Solution {
    public int smallestDivisor(int[] nums, int threshold) {
        int lo = 1, hi = 1;
        for (int x : nums) hi = Math.max(hi, x);
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (total(nums, mid) <= threshold) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    // The sum can reach n * 10^6, which overflows int, so accumulate in a long.
    private long total(int[] nums, int d) {
        long s = 0;
        for (int x : nums) s += (x + d - 1) / d;
        return s;
    }
}
