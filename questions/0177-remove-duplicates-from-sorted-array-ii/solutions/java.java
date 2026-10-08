class Solution {
    public int[] removeDuplicatesKeepTwo(int[] nums) {
        int[] a = nums.clone();
        int k = 0;
        for (int i = 0; i < a.length; i++) {
            int x = a[i];
            if (k < 2 || a[k - 2] != x) {
                a[k++] = x;
            }
        }
        return Arrays.copyOf(a, k);
    }
}
