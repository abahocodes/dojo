class Solution {
    public int[] mergeSorted(int[] nums1, int[] nums2) {
        int m = nums1.length, n = nums2.length;
        int[] out = Arrays.copyOf(nums1, m + n);
        int i = m - 1, j = n - 1, w = m + n - 1;
        while (j >= 0) {
            if (i >= 0 && out[i] > nums2[j]) {
                out[w--] = out[i--];
            } else {
                out[w--] = nums2[j--];
            }
        }
        return out;
    }
}
