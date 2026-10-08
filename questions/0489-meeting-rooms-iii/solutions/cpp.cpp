class Solution {
public:
    int mostBooked(int n, vector<vector<int>>& meetings) {
        vector<vector<int>> sorted = meetings;
        sort(sorted.begin(), sorted.end());

        priority_queue<int, vector<int>, greater<int>> freeRooms;
        for (int r = 0; r < n; r++) freeRooms.push(r);
        // Busy rooms as (end time, room), earliest end first, then lowest room.
        // End times can pass 2^31 after repeated delays, so they are long long.
        priority_queue<pair<long long, int>, vector<pair<long long, int>>, greater<pair<long long, int>>> busy;
        vector<int> count(n, 0);

        for (auto& m : sorted) {
            long long start = m[0];
            long long end = m[1];
            while (!busy.empty() && busy.top().first <= start) {
                freeRooms.push(busy.top().second);
                busy.pop();
            }
            int room;
            if (!freeRooms.empty()) {
                room = freeRooms.top();
                freeRooms.pop();
                busy.push({end, room});
            } else {
                // Wait for the earliest room; it keeps the meeting's duration.
                auto [freeAt, r] = busy.top();
                busy.pop();
                room = r;
                busy.push({freeAt + end - start, room});
            }
            count[room]++;
        }

        int best = 0;
        for (int r = 1; r < n; r++) if (count[r] > count[best]) best = r;
        return best;
    }
};
