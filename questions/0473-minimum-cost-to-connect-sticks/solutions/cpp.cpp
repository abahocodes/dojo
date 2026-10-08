class Solution {
public:
    long long connectSticks(vector<int>& sticks) {
        priority_queue<long long, vector<long long>, greater<long long>> heap(sticks.begin(), sticks.end());
        long long total = 0;
        while (heap.size() > 1) {
            long long a = heap.top();
            heap.pop();
            long long b = heap.top();
            heap.pop();
            total += a + b;
            heap.push(a + b);
        }
        return total;
    }
};
