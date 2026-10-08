class Solution {
    public int[] canSeePersonsCount(int[] heights) {
        int n = heights.length;
        int[] answer = new int[n];
        int[] stack = new int[n]; // heights, decreasing from bottom to top
        int top = 0;
        for (int i = n - 1; i >= 0; i--) {
            int seen = 0;
            while (top > 0 && stack[top - 1] < heights[i]) {
                top--;
                seen++;
            }
            if (top > 0) seen++; // the first taller person
            answer[i] = seen;
            stack[top++] = heights[i];
        }
        return answer;
    }
}
