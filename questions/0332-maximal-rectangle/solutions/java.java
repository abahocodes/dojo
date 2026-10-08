class Solution {
    public int maximalRectangle(String[] matrix) {
        int cols = matrix[0].length();
        int[] heights = new int[cols + 1]; // heights[cols] stays 0
        int[] stack = new int[cols + 1];
        int best = 0;
        for (String row : matrix) {
            for (int c = 0; c < cols; c++) {
                heights[c] = row.charAt(c) == '1' ? heights[c] + 1 : 0;
            }
            int top = 0;
            for (int i = 0; i <= cols; i++) {
                while (top > 0 && heights[stack[top - 1]] >= heights[i]) {
                    int h = heights[stack[--top]];
                    int left = top > 0 ? stack[top - 1] : -1;
                    best = Math.max(best, h * (i - left - 1));
                }
                stack[top++] = i;
            }
        }
        return best;
    }
}
