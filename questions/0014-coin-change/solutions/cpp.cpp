class Solution {
public:
    int coinChange(vector<int>& coins, int amount) {
        const int inf = amount + 1;
        vector<int> best(amount + 1, inf);
        best[0] = 0;
        for (int total = 1; total <= amount; total++) {
            for (int coin : coins) {
                if (coin <= total && best[total - coin] + 1 < best[total]) {
                    best[total] = best[total - coin] + 1;
                }
            }
        }
        return best[amount] != inf ? best[amount] : -1;
    }
};
