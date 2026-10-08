class Solution {
public:
    int minEatingSpeed(vector<int>& piles, int h) {
        int lo = 1, hi = *max_element(piles.begin(), piles.end());
        while (lo < hi) {
            int v = lo + (hi - lo) / 2;
            long long hours = 0;
            for (int p : piles) {
                hours += (p + (long long) v - 1) / v;
            }
            if (hours <= h) {
                hi = v;
            } else {
                lo = v + 1;
            }
        }
        return lo;
    }
};
