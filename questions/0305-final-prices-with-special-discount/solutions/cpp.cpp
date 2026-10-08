class Solution {
public:
    vector<int> finalPrices(vector<int>& prices) {
        vector<int> result = prices;
        vector<int> stack; // indices awaiting a discount; prices increase
        for (int j = 0; j < (int)prices.size(); j++) {
            while (!stack.empty() && prices[stack.back()] >= prices[j]) {
                result[stack.back()] -= prices[j];
                stack.pop_back();
            }
            stack.push_back(j);
        }
        return result;
    }
};
