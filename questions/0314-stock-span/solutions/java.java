class Solution {
    public int[] stockSpan(int[] prices) {
        int n = prices.length;
        int[] result = new int[n];
        int[] stack = new int[n];
        int top = 0;
        for (int i = 0; i < n; i++) {
            while (top > 0 && prices[stack[top - 1]] <= prices[i]) top--;
            result[i] = top > 0 ? i - stack[top - 1] : i + 1;
            stack[top++] = i;
        }
        return result;
    }
}
