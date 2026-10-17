class Solution {
    public int maxProfit(int[] prices) {
        // Best profit at the end of each day, by state:
        // hold = owning a share, sold = sold today, rest = no share and free to buy.
        int hold = Integer.MIN_VALUE / 2, sold = 0, rest = 0;
        for (int p : prices) {
            int prevSold = sold;
            sold = hold + p;
            hold = Math.max(hold, rest - p);
            rest = Math.max(rest, prevSold);
        }
        return Math.max(sold, rest);
    }
}
