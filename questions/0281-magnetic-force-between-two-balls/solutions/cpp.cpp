class Solution {
public:
    int maxMinDistance(vector<int>& position, int m) {
        vector<int> pos(position);
        sort(pos.begin(), pos.end());
        int lo = 1, hi = (pos.back() - pos.front()) / (m - 1);
        while (lo < hi) {
            int mid = lo + (hi - lo + 1) / 2;
            if (fits(pos, m, mid)) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }

private:
    // Greedily drop a ball in the leftmost basket at least `gap` past the last one.
    bool fits(const vector<int>& pos, int m, int gap) {
        int placed = 1, last = pos[0];
        for (size_t i = 1; i < pos.size(); i++) {
            if (pos[i] - last >= gap) {
                placed++;
                last = pos[i];
                if (placed == m) return true;
            }
        }
        return false;
    }
};
