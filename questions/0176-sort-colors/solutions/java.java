class Solution {
    public int[] sortColors(int[] nums) {
        int[] a = nums.clone();
        int low = 0, mid = 0, high = a.length - 1;
        while (mid <= high) {
            if (a[mid] == 0) {
                int t = a[low]; a[low] = a[mid]; a[mid] = t;
                low++;
                mid++;
            } else if (a[mid] == 1) {
                mid++;
            } else {
                int t = a[high]; a[high] = a[mid]; a[mid] = t;
                high--;
            }
        }
        return a;
    }
}
