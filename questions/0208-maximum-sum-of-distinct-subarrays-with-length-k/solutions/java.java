class Solution {
    public long maximumSubarraySumDistinct(int[] nums, int k) {
        int[] count = new int[100001];
        int dup = 0;
        long window = 0;
        long best = 0;
        for (int i = 0; i < nums.length; i++) {
            int value = nums[i];
            window += value;
            if (++count[value] == 2) dup++;
            if (i >= k) {
                int old = nums[i - k];
                window -= old;
                if (--count[old] == 1) dup--;
            }
            if (i >= k - 1 && dup == 0) best = Math.max(best, window);
        }
        return best;
    }
}
