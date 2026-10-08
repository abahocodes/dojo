class Solution {
public:
    int smallestDivisor(vector<int>& nums, int threshold) {
        int lo = 1, hi = *max_element(nums.begin(), nums.end());
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (total(nums, mid) <= threshold) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

private:
    // The sum can reach n * 10^6, which overflows int, so accumulate in a long long.
    long long total(const vector<int>& nums, int d) {
        long long s = 0;
        for (int x : nums) s += (x + d - 1) / d;
        return s;
    }
};
