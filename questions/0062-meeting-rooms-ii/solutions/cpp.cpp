class Solution {
public:
    int minMeetingRooms(vector<vector<int>>& intervals) {
        vector<int> starts, ends;
        for (const auto& iv : intervals) {
            starts.push_back(iv[0]);
            ends.push_back(iv[1]);
        }
        sort(starts.begin(), starts.end());
        sort(ends.begin(), ends.end());
        int rooms = 0;
        size_t j = 0; // ends[j] is the earliest end time that hasn't freed a room yet
        for (int s : starts) {
            if (s >= ends[j]) j++;
            else rooms++;
        }
        return rooms;
    }
};
