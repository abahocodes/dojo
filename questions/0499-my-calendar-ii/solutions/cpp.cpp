class Solution {
public:
    vector<bool> bookCalendarDouble(vector<vector<int>>& bookings) {
        vector<pair<int, int>> booked;   // every accepted booking
        vector<pair<int, int>> overlaps; // stretches already covered twice
        vector<bool> result;
        for (auto& b : bookings) {
            int start = b[0], end = b[1];
            bool ok = true;
            for (auto& [s, e] : overlaps) {
                if (max(start, s) < min(end, e)) {
                    ok = false;
                    break;
                }
            }
            if (ok) {
                for (auto& [s, e] : booked) {
                    int lo = max(start, s), hi = min(end, e);
                    if (lo < hi) overlaps.push_back({lo, hi});
                }
                booked.push_back({start, end});
            }
            result.push_back(ok);
        }
        return result;
    }
};
