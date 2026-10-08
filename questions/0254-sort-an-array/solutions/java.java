class Solution {
    public int[] sortArray(int[] nums) {
        int n = nums.length;
        int[] a = nums.clone();
        int[] buf = new int[n];
        for (int width = 1; width < n; width *= 2) {
            for (int lo = 0; lo < n; lo += 2 * width) {
                int mid = Math.min(lo + width, n);
                int hi = Math.min(lo + 2 * width, n);
                int i = lo, j = mid, k = lo;
                while (i < mid && j < hi) buf[k++] = a[i] <= a[j] ? a[i++] : a[j++];
                while (i < mid) buf[k++] = a[i++];
                while (j < hi) buf[k++] = a[j++];
            }
            int[] t = a;
            a = buf;
            buf = t;
        }
        return a;
    }
}
