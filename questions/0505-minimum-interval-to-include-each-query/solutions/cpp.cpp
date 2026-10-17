class Solution {
public:
    vector<int> minInterval(vector<vector<int>>& intervals, vector<int>& queries) {
        vector<vector<int>> sorted = intervals;
        sort(sorted.begin(), sorted.end());
        vector<int> order(queries.size());
        for (int i = 0; i < (int)order.size(); i++) order[i] = i;
        sort(order.begin(), order.end(), [&](int a, int b) { return queries[a] < queries[b]; });

        vector<int> answer(queries.size(), -1);
        // Min-heap of {length, right end} for intervals that start at or before q.
        priority_queue<pair<int, int>, vector<pair<int, int>>, greater<pair<int, int>>> heap;
        size_t i = 0;
        for (int qi : order) {
            int q = queries[qi];
            while (i < sorted.size() && sorted[i][0] <= q) {
                heap.push({sorted[i][1] - sorted[i][0] + 1, sorted[i][1]});
                i++;
            }
            // Intervals that end before q can never contain a later (larger) query.
            while (!heap.empty() && heap.top().second < q) heap.pop();
            if (!heap.empty()) answer[qi] = heap.top().first;
        }
        return answer;
    }
};
