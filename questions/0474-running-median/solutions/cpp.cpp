class Solution {
public:
    vector<double> runningMedian(vector<int>& nums) {
        priority_queue<int> low;                             // smaller half (max-heap)
        priority_queue<int, vector<int>, greater<int>> high; // larger half (min-heap)
        vector<double> medians;
        medians.reserve(nums.size());
        for (int x : nums) {
            if (low.empty() || x <= low.top()) low.push(x);
            else high.push(x);
            // Keep low.size() == high.size() or low.size() == high.size() + 1.
            if (low.size() > high.size() + 1) {
                high.push(low.top());
                low.pop();
            } else if (high.size() > low.size()) {
                low.push(high.top());
                high.pop();
            }
            if (low.size() > high.size()) medians.push_back(low.top());
            else medians.push_back(((double)low.top() + high.top()) / 2.0);
        }
        return medians;
    }
};
