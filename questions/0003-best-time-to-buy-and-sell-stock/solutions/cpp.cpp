class Solution {
public:
    int maxProfit(vector<int>& prices) {
        int lowest = prices[0];
        int best = 0;
        for (int p : prices) {
            best = max(best, p - lowest);
            lowest = min(lowest, p);
        }
        return best;
    }
};
