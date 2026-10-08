class Solution {
    public int maxWidthRamp(int[] nums) {
        int n = nums.length;
        int[] stack = new int[n];
        int top = -1;
        for (int i = 0; i < n; i++) {
            if (top < 0 || nums[i] < nums[stack[top]]) stack[++top] = i;
        }
        int best = 0;
        for (int j = n - 1; j >= 0; j--) {
            while (top >= 0 && nums[stack[top]] <= nums[j]) {
                best = Math.max(best, j - stack[top--]);
            }
        }
        return best;
    }
}
