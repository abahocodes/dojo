class Solution {
public:
    int eraseOverlapIntervals(vector<vector<int>>& intervals) {
        vector<vector<int>> byEnd = intervals;
        // keeping the interval that ends first leaves the most room for the rest
        sort(byEnd.begin(), byEnd.end(),
             [](const vector<int>& a, const vector<int>& b) { return a[1] < b[1]; });
        int removed = 0;
        long long lastEnd = LLONG_MIN;
        for (const auto& iv : byEnd) {
            if (iv[0] >= lastEnd) lastEnd = iv[1];
            else removed++;
        }
        return removed;
    }
};
