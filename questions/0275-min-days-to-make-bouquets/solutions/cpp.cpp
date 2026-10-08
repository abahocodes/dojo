class Solution {
public:
    int minDays(vector<int>& bloomDay, int m, int k) {
        if ((long long)m * k > (long long)bloomDay.size()) {
            return -1;
        }
        int lo = *min_element(bloomDay.begin(), bloomDay.end());
        int hi = *max_element(bloomDay.begin(), bloomDay.end());
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (bouquets(bloomDay, k, mid) >= m) {
                hi = mid;
            } else {
                lo = mid + 1;
            }
        }
        return lo;
    }

private:
    int bouquets(const vector<int>& bloomDay, int k, int day) {
        int made = 0, run = 0;
        for (int b : bloomDay) {
            if (b <= day) {
                run++;
                if (run == k) {
                    made++;
                    run = 0;
                }
            } else {
                run = 0;
            }
        }
        return made;
    }
};
