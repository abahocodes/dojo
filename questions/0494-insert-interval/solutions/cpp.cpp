class Solution {
public:
    vector<vector<int>> insertInterval(vector<vector<int>>& intervals, vector<int>& newInterval) {
        vector<vector<int>> result;
        int start = newInterval[0], end = newInterval[1];
        size_t i = 0, n = intervals.size();
        // Intervals that end strictly before the new one starts.
        while (i < n && intervals[i][1] < start) result.push_back(intervals[i++]);
        // Intervals that overlap or touch the new one: absorb them.
        while (i < n && intervals[i][0] <= end) {
            start = min(start, intervals[i][0]);
            end = max(end, intervals[i][1]);
            i++;
        }
        result.push_back({start, end});
        // Intervals that start strictly after the merged one ends.
        while (i < n) result.push_back(intervals[i++]);
        return result;
    }
};
