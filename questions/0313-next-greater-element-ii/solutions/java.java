class Solution {
    public int[] nextGreaterCircular(int[] nums) {
        int n = nums.length;
        int[] result = new int[n];
        Arrays.fill(result, -1);
        int[] stack = new int[n];
        int top = 0;
        for (int j = 0; j < 2 * n; j++) {
            int x = nums[j % n];
            while (top > 0 && nums[stack[top - 1]] < x) {
                result[stack[--top]] = x;
            }
            if (j < n) stack[top++] = j;
        }
        return result;
    }
}
