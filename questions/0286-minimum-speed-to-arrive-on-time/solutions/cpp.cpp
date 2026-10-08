class Solution {
    bool onTime(const vector<int>& dist, long long total, int speed) {
        long long whole = 0;
        for (size_t i = 0; i + 1 < dist.size(); i++) whole += (dist[i] + speed - 1) / speed;
        long long rest = total - whole * 100;
        long long last = dist.back() * 100LL;
        return rest >= 0 && (rest >= last || last <= rest * speed);
    }

public:
    int minSpeedOnTime(vector<int>& dist, double hour) {
        long long total = llround(hour * 100);
        int lo = 1, hi = 10000000;
        if (!onTime(dist, total, hi)) return -1;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (onTime(dist, total, mid)) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }
};
