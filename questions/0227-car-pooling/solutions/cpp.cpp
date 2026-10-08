class Solution {
public:
    bool carPooling(vector<vector<int>>& trips, int capacity) {
        // change[x] = passengers boarding at km x minus passengers leaving at km x
        vector<int> change(1001, 0);
        for (auto& t : trips) {
            change[t[1]] += t[0];
            change[t[2]] -= t[0];
        }
        int load = 0;
        for (int delta : change) {
            load += delta;
            if (load > capacity) return false;
        }
        return true;
    }
};
