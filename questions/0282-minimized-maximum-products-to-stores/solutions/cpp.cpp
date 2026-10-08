class Solution {
public:
    int minimizedMaximum(int n, vector<int>& quantities) {
        int lo = 1, hi = *max_element(quantities.begin(), quantities.end());
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (storesNeeded(quantities, mid) <= n) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

private:
    // With cap = 1 the total reaches m * 10^5, beyond int range, so use long long.
    long long storesNeeded(const vector<int>& quantities, int cap) {
        long long s = 0;
        for (int q : quantities) s += (q + cap - 1) / cap;
        return s;
    }
};
