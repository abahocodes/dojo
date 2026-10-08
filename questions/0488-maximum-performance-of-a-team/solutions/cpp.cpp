class Solution {
public:
    int maxPerformance(int n, vector<int>& speed, vector<int>& efficiency, int k) {
        vector<int> order(n);
        iota(order.begin(), order.end(), 0);
        sort(order.begin(), order.end(), [&](int a, int b) { return efficiency[a] > efficiency[b]; });

        priority_queue<int, vector<int>, greater<int>> heap; // speeds in the current team
        long long totalSpeed = 0;
        long long best = 0; // up to 10^18: fits in a long long
        for (int i : order) {
            // efficiency[i] is the smallest efficiency so far: it is the team minimum.
            heap.push(speed[i]);
            totalSpeed += speed[i];
            if ((int)heap.size() > k) {
                totalSpeed -= heap.top();
                heap.pop();
            }
            best = max(best, totalSpeed * efficiency[i]);
        }
        return (int)(best % 1000000007LL);
    }
};
