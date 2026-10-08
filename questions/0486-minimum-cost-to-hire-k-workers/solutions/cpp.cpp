class Solution {
public:
    double mincostToHireWorkers(vector<int>& quality, vector<int>& wage, int k) {
        int n = quality.size();
        // Sort workers by wage/quality ratio, compared exactly by cross-multiplying.
        vector<int> order(n);
        iota(order.begin(), order.end(), 0);
        sort(order.begin(), order.end(), [&](int a, int b) {
            return (long long)wage[a] * quality[b] < (long long)wage[b] * quality[a];
        });

        priority_queue<int> heap; // qualities in the current group
        long long totalQuality = 0;
        double best = numeric_limits<double>::max();
        for (int i : order) {
            heap.push(quality[i]);
            totalQuality += quality[i];
            if ((int)heap.size() > k) {
                totalQuality -= heap.top(); // drop the largest quality
                heap.pop();
            }
            if ((int)heap.size() == k) {
                // Worker i has the largest ratio so far and sets the pay rate.
                best = min(best, (double)(totalQuality * wage[i]) / quality[i]);
            }
        }
        return best;
    }
};
