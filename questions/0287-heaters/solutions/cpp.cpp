class Solution {
public:
    int findRadius(vector<int>& houses, vector<int>& heaters) {
        vector<int> hs(heaters);
        sort(hs.begin(), hs.end());
        int best = 0;
        for (int x : houses) {
            auto it = lower_bound(hs.begin(), hs.end(), x);
            int near = INT_MAX;
            if (it != hs.end()) near = *it - x;
            if (it != hs.begin()) near = min(near, x - *prev(it));
            best = max(best, near);
        }
        return best;
    }
};
