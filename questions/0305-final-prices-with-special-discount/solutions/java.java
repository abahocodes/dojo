class Solution {
    public int[] finalPrices(int[] prices) {
        int n = prices.length;
        int[] result = prices.clone();
        int[] stack = new int[n]; // indices awaiting a discount; prices increase
        int top = 0;
        for (int j = 0; j < n; j++) {
            while (top > 0 && prices[stack[top - 1]] >= prices[j]) {
                result[stack[--top]] -= prices[j];
            }
            stack[top++] = j;
        }
        return result;
    }
}
