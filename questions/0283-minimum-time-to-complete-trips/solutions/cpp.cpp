class Solution {
public:
    long long minimumTime(vector<int>& time, int totalTrips) {
        long long lo = 1;
        long long hi = (long long)*min_element(time.begin(), time.end()) * totalTrips;
        while (lo < hi) {
            long long mid = lo + (hi - lo) / 2;
            if (enough(time, totalTrips, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

private:
    // Stop as soon as the target is reached so the running count cannot overflow.
    bool enough(const vector<int>& time, int totalTrips, long long t) {
        long long done = 0;
        for (int x : time) {
            done += t / x;
            if (done >= totalTrips) return true;
        }
        return false;
    }
};
