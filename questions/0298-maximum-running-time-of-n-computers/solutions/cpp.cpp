class Solution {
public:
    long long maxRunTime(int n, vector<int>& batteries) {
        long long total = 0;
        for (int b : batteries) total += b;
        long long lo = 0, hi = total / n;
        while (lo < hi) {
            long long mid = lo + (hi - lo + 1) / 2;
            if (canRun(n, batteries, mid)) lo = mid;
            else hi = mid - 1;
        }
        return lo;
    }

private:
    bool canRun(int n, const vector<int>& batteries, long long minutes) {
        long long usable = 0;
        for (int b : batteries) usable += min((long long)b, minutes);
        return usable >= (long long)n * minutes;
    }
};
