class Solution {
public:
    int maximumRemovals(string& s, string& p, vector<int>& removable) {
        int r = removable.size();
        vector<int> removedAt(s.size(), r);
        for (int step = 0; step < r; step++) removedAt[removable[step]] = step;
        int lo = 0, hi = r;
        while (lo < hi) {
            int mid = lo + (hi - lo + 1) / 2;
            size_t j = 0;
            for (size_t i = 0; i < s.size() && j < p.size(); i++) {
                if (removedAt[i] >= mid && s[i] == p[j]) j++;
            }
            if (j == p.size()) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
};
