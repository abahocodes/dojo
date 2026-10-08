class Solution {
public:
    int maximumGap(vector<int>& nums) {
        int n = nums.size();
        if (n < 2) return 0;
        auto [loIt, hiIt] = minmax_element(nums.begin(), nums.end());
        int lo = *loIt, hi = *hiIt;
        if (lo == hi) return 0;
        int size = max(1, (hi - lo) / (n - 1));
        int count = (hi - lo) / size + 1;
        vector<int> bucketMin(count, INT_MAX), bucketMax(count, -1);
        for (int v : nums) {
            int b = (v - lo) / size;
            bucketMin[b] = min(bucketMin[b], v);
            bucketMax[b] = max(bucketMax[b], v);
        }
        int best = 0, prev = lo;
        for (int b = 0; b < count; b++) {
            if (bucketMax[b] < 0) continue; // empty
            best = max(best, bucketMin[b] - prev);
            prev = bucketMax[b];
        }
        return best;
    }
};
