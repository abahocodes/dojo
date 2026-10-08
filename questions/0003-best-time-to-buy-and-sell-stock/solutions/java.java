class Solution {
    public int maxProfit(int[] prices) {
        int lowest = prices[0];
        int best = 0;
        for (int p : prices) {
            best = Math.max(best, p - lowest);
            lowest = Math.min(lowest, p);
        }
        return best;
    }
}
