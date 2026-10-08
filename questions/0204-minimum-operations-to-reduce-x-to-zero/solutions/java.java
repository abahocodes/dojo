class Solution {
    public int minOperationsReduceX(int[] nums, int x) {
        long total = 0;
        for (int v : nums) total += v;
        long target = total - x;
        if (target < 0) return -1;
        int best = -1;
        long window = 0;
        int left = 0;
        for (int right = 0; right < nums.length; right++) {
            window += nums[right];
            while (window > target) window -= nums[left++];
            if (window == target) best = Math.max(best, right - left + 1);
        }
        return best == -1 ? -1 : nums.length - best;
    }
}
