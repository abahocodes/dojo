class Solution {
public:
    int maxIceCream(vector<int>& costs, int coins) {
        int maxCost = *max_element(costs.begin(), costs.end());
        vector<int> count(maxCost + 1, 0);
        for (int c : costs) count[c]++;
        int bought = 0;
        for (int price = 1; price <= maxCost; price++) {
            if (count[price] == 0) continue;
            int take = min(count[price], coins / price);
            bought += take;
            coins -= take * price;
            if (take < count[price]) break;
        }
        return bought;
    }
};
