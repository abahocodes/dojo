class Solution {
public:
    long long totalCost(vector<int>& costs, int k, int candidates) {
        using Worker = pair<int, int>;  // cost, index
        priority_queue<Worker, vector<Worker>, greater<Worker>> left, right;
        int i = 0, j = (int)costs.size() - 1;
        long long total = 0;
        for (int round = 0; round < k; round++) {
            while ((int)left.size() < candidates && i <= j) { left.push({costs[i], i}); i++; }
            while ((int)right.size() < candidates && i <= j) { right.push({costs[j], j}); j--; }
            if (right.empty() || (!left.empty() && left.top() < right.top())) {
                total += left.top().first;
                left.pop();
            } else {
                total += right.top().first;
                right.pop();
            }
        }
        return total;
    }
};
