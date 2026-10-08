class Solution {
public:
    int findMaximizedCapital(int k, int w, vector<int>& profits, vector<int>& capital) {
        int n = profits.size();
        vector<int> order(n);
        iota(order.begin(), order.end(), 0);
        sort(order.begin(), order.end(), [&](int a, int b) { return capital[a] < capital[b]; });
        priority_queue<int> affordable;  // max-heap of profits
        int p = 0;
        for (int round = 0; round < k; round++) {
            while (p < n && capital[order[p]] <= w) affordable.push(profits[order[p++]]);
            if (affordable.empty()) break;
            w += affordable.top();
            affordable.pop();
        }
        return w;
    }
};
