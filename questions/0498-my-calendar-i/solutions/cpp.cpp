class Solution {
public:
    vector<bool> bookCalendar(vector<vector<int>>& bookings) {
        // start -> end of every accepted booking.
        map<int, int> calendar;
        vector<bool> result;
        for (auto& b : bookings) {
            int start = b[0], end = b[1];
            // The latest booking that begins before `end` is the only one that can overlap.
            auto it = calendar.lower_bound(end);
            if (it != calendar.begin() && prev(it)->second > start) {
                result.push_back(false);
                continue;
            }
            calendar[start] = end;
            result.push_back(true);
        }
        return result;
    }
};
