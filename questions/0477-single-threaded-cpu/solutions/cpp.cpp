class Solution {
public:
    vector<int> getOrder(vector<vector<int>>& tasks) {
        int n = tasks.size();
        vector<int> byEnqueue(n);
        iota(byEnqueue.begin(), byEnqueue.end(), 0);
        sort(byEnqueue.begin(), byEnqueue.end(), [&](int a, int b) {
            return tasks[a][0] != tasks[b][0] ? tasks[a][0] < tasks[b][0] : a < b;
        });
        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> ready;
        vector<int> order;
        order.reserve(n);
        long long time = 0;
        int p = 0;
        while ((int)order.size() < n) {
            if (ready.empty() && time < tasks[byEnqueue[p]][0]) time = tasks[byEnqueue[p]][0];
            while (p < n && tasks[byEnqueue[p]][0] <= time) {
                int i = byEnqueue[p++];
                ready.push({tasks[i][1], i});
            }
            auto [duration, i] = ready.top();
            ready.pop();
            time += duration;
            order.push_back(i);
        }
        return order;
    }
};
