class Solution {
    public double findMedianSortedArrays(int[] nums1, int[] nums2) {
        if (nums1.length > nums2.length) {
            int[] tmp = nums1;
            nums1 = nums2;
            nums2 = tmp;
        }
        int m = nums1.length, n = nums2.length;
        int half = (m + n + 1) / 2;
        int lo = 0, hi = m;
        while (lo <= hi) {
            int i = (lo + hi) / 2;
            int j = half - i;
            double left1 = i > 0 ? nums1[i - 1] : Double.NEGATIVE_INFINITY;
            double right1 = i < m ? nums1[i] : Double.POSITIVE_INFINITY;
            double left2 = j > 0 ? nums2[j - 1] : Double.NEGATIVE_INFINITY;
            double right2 = j < n ? nums2[j] : Double.POSITIVE_INFINITY;
            if (left1 > right2) {
                hi = i - 1;
            } else if (left2 > right1) {
                lo = i + 1;
            } else {
                if ((m + n) % 2 == 1) {
                    return Math.max(left1, left2);
                }
                return (Math.max(left1, left2) + Math.min(right1, right2)) / 2;
            }
        }
        return 0.0;
    }
}
