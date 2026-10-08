class Solution {
public:
    long long maxMinPower(vector<int>& stations, int r, int k) {
        int n = stations.size();
        // power[i] = sum of stations in [i - r, i + r], via a sliding window.
        vector<long long> power(n);
        long long window = 0;
        for (int i = 0; i < min(n, r + 1); i++) window += stations[i];
        for (int i = 0; i < n; i++) {
            power[i] = window;
            if (i + r + 1 < n) window += stations[i + r + 1];
            if (i - r >= 0) window -= stations[i - r];
        }
        long long lo = *min_element(power.begin(), power.end());
        long long hi = lo + k;
        vector<long long> added(n + 1);
        auto feasible = [&](long long target) {
            fill(added.begin(), added.end(), 0LL);
            long long extra = 0, used = 0;
            for (int i = 0; i < n; i++) {
                extra += added[i];
                long long have = power[i] + extra;
                if (have < target) {
                    long long need = target - have;
                    used += need;
                    if (used > k) return false;
                    extra += need;
                    added[min<long long>(n, (long long)i + 2LL * r + 1)] -= need;
                }
            }
            return true;
        };
        while (lo < hi) {
            long long mid = lo + (hi - lo + 1) / 2;
            if (feasible(mid)) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }
};
