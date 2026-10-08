class Solution {
    public int maxProduct(int[] nums) {
        int best = nums[0], hi = nums[0], lo = nums[0];
        for (int i = 1; i < nums.length; i++) {
            int x = nums[i];
            if (x < 0) {
                int tmp = hi;
                hi = lo;
                lo = tmp;
            }
            hi = Math.max(x, hi * x);
            lo = Math.min(x, lo * x);
            best = Math.max(best, hi);
        }
        return best;
    }
}
