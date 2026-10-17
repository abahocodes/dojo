class Solution {
public:
    int maxProfit(vector<int>& prices) {
        // Best profit at the end of each day, by state:
        // hold = owning a share, sold = sold today, rest = no share and free to buy.
        int hold = INT_MIN / 2, sold = 0, rest = 0;
        for (int p : prices) {
            int prevSold = sold;
            sold = hold + p;
            hold = max(hold, rest - p);
            rest = max(rest, prevSold);
        }
        return max(sold, rest);
    }
};
