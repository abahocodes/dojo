class Solution {
public:
    vector<vector<int>> getSkyline(vector<vector<int>>& buildings) {
        // Events (x, -height, right): starts carry -height, ends carry 0.
        // Sorted by x, then starts before ends, tallest start first.
        vector<array<int, 3>> events;
        events.reserve(2 * buildings.size());
        for (auto& b : buildings) {
            events.push_back({b[0], -b[2], b[1]});
            events.push_back({b[1], 0, 0});
        }
        sort(events.begin(), events.end());

        // Max-heap of (height, right) for buildings that may still stand.
        priority_queue<pair<int, int>> live;
        vector<vector<int>> result;
        for (auto& [x, negHeight, right] : events) {
            // Lazily drop buildings that ended at or before x.
            while (!live.empty() && live.top().second <= x) live.pop();
            if (negHeight != 0) live.push({-negHeight, right});
            // The first event at x already settles the height at x.
            int current = live.empty() ? 0 : live.top().first;
            if (result.empty() || result.back()[1] != current) {
                result.push_back({x, current});
            }
        }
        return result;
    }
};
