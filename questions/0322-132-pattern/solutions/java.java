class Solution {
    public boolean find132Pattern(int[] nums) {
        int third = Integer.MIN_VALUE; // every value is > -2^31, so this means "none yet"
        int[] stack = new int[nums.length];
        int top = 0;
        for (int i = nums.length - 1; i >= 0; i--) {
            int x = nums[i];
            if (x < third) return true;
            while (top > 0 && stack[top - 1] < x) third = stack[--top];
            stack[top++] = x;
        }
        return false;
    }
}
