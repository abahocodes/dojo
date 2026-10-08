class Solution {
    public int arrayPairSum(int[] nums) {
        int[] a = nums.clone();
        Arrays.sort(a);
        int total = 0;
        for (int i = 0; i < a.length; i += 2) total += a[i];
        return total;
    }
}
