class Solution {
    public long subArrayRanges(int[] nums) {
        return total(nums, 1) - total(nums, -1);
    }

    // sign = 1 sums subarray maxima, sign = -1 sums subarray minima.
    private long total(int[] nums, int sign) {
        int n = nums.length;
        int[] stack = new int[n + 1];
        int top = -1;
        long result = 0;
        for (int i = 0; i <= n; i++) {
            while (top >= 0 && (i == n || (long) sign * nums[stack[top]] <= (long) sign * nums[i])) {
                int j = stack[top--];
                int left = top >= 0 ? stack[top] : -1;
                result += (long) nums[j] * (j - left) * (i - j);
            }
            stack[++top] = i;
        }
        return result;
    }
}
