class Solution {
    public int maxSubArray(int[] nums) {
        int current = nums[0], best = nums[0];
        for (int i = 1; i < nums.length; i++) {
            int x = nums[i];
            current = Math.max(x, current + x);
            best = Math.max(best, current);
        }
        return best;
    }
}
