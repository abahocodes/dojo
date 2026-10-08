class Solution {
    public int largestRectangleArea(int[] heights) {
        int n = heights.length;
        int[] stack = new int[n + 1];
        int top = 0;
        int best = 0;
        for (int i = 0; i <= n; i++) {
            int h = i < n ? heights[i] : 0;
            while (top > 0 && heights[stack[top - 1]] >= h) {
                int height = heights[stack[--top]];
                int left = top > 0 ? stack[top - 1] : -1;
                best = Math.max(best, height * (i - left - 1));
            }
            stack[top++] = i;
        }
        return best;
    }
}
