class Solution {
    public int triangleNumber(int[] nums) {
        int[] a = nums.clone();
        Arrays.sort(a);
        int count = 0;
        for (int k = a.length - 1; k >= 2; k--) {
            int i = 0, j = k - 1;
            while (i < j) {
                if (a[i] + a[j] > a[k]) {
                    count += j - i;
                    j--;
                } else {
                    i++;
                }
            }
        }
        return count;
    }
}
