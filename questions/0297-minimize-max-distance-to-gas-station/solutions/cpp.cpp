class Solution {
public:
    double minmaxGasDist(vector<int>& stations, int k) {
        double lo = 0, hi = 0;
        for (size_t i = 1; i < stations.size(); i++) {
            hi = max(hi, (double)(stations[i] - stations[i - 1]));
        }
        for (int iter = 0; iter < 100; iter++) {
            double mid = (lo + hi) / 2;
            if (fits(stations, k, mid)) hi = mid;
            else lo = mid;
        }
        return hi;
    }

private:
    bool fits(const vector<int>& stations, int k, double limit) {
        long long added = 0;
        for (size_t i = 1; i < stations.size(); i++) {
            added += (long long)((stations[i] - stations[i - 1]) / limit);
            if (added > k) return false;
        }
        return true;
    }
};
