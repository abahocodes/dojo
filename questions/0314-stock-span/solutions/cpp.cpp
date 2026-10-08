class Solution {
public:
    vector<int> stockSpan(vector<int>& prices) {
        int n = prices.size();
        vector<int> result(n);
        vector<int> stack;
        for (int i = 0; i < n; i++) {
            while (!stack.empty() && prices[stack.back()] <= prices[i]) stack.pop_back();
            result[i] = stack.empty() ? i + 1 : i - stack.back();
            stack.push_back(i);
        }
        return result;
    }
};
