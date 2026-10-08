class Solution {
    public int[] sortedSquares(int[] nums) {
        int n = nums.length;
        int[] out = new int[n];
        int lo = 0, hi = n - 1;
        for (int w = n - 1; w >= 0; w--) {
            if (Math.abs(nums[lo]) > Math.abs(nums[hi])) {
                out[w] = nums[lo] * nums[lo];
                lo++;
            } else {
                out[w] = nums[hi] * nums[hi];
                hi--;
            }
        }
        return out;
    }
}
