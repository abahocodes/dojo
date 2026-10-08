class Solution {
    public long kthSmallestProduct(int[] nums1, int[] nums2, long k) {
        int n1 = nums1.length, n2 = nums2.length;
        long[] corners = {
            (long) nums1[0] * nums2[0], (long) nums1[0] * nums2[n2 - 1],
            (long) nums1[n1 - 1] * nums2[0], (long) nums1[n1 - 1] * nums2[n2 - 1]
        };
        long lo = Long.MAX_VALUE, hi = Long.MIN_VALUE;
        for (long c : corners) {
            lo = Math.min(lo, c);
            hi = Math.max(hi, c);
        }
        while (lo < hi) {
            long mid = Math.floorDiv(lo + hi, 2L);
            if (countAtMost(nums1, nums2, mid) >= k) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

    private long countAtMost(int[] nums1, int[] nums2, long x) {
        long total = 0;
        for (int a : nums1) total += countFor(a, nums2, x);
        return total;
    }

    // Number of j with a * nums2[j] <= x.
    private int countFor(long a, int[] nums2, long x) {
        int n2 = nums2.length;
        if (a == 0) return x >= 0 ? n2 : 0;
        int lo = 0, hi = n2;
        if (a > 0) {
            while (lo < hi) {
                int mid = (lo + hi) >>> 1;
                if (a * nums2[mid] <= x) lo = mid + 1;
                else hi = mid;
            }
            return lo;
        }
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (a * nums2[mid] <= x) hi = mid;
            else lo = mid + 1;
        }
        return n2 - lo;
    }
}
