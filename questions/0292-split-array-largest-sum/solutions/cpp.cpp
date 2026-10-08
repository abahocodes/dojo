class Solution {
public:
    int splitArray(vector<int>& nums, int k) {
        long long lo = 0, hi = 0;
        for (int x : nums) {
            lo = max(lo, (long long)x);
            hi += x;
        }
        while (lo < hi) {
            long long mid = lo + (hi - lo) / 2;
            if (piecesNeeded(nums, mid) <= k) hi = mid;
            else lo = mid + 1;
        }
        return (int)lo;
    }

private:
    int piecesNeeded(const vector<int>& nums, long long cap) {
        int pieces = 1;
        long long current = 0;
        for (int x : nums) {
            if (current + x > cap) {
                pieces++;
                current = x;
            } else {
                current += x;
            }
        }
        return pieces;
    }
};
