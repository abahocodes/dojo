class Solution {
public:
    int minMovesKOnes(vector<int>& nums, int k) {
        vector<long long> q;
        for (int i = 0; i < (int)nums.size(); i++) {
            if (nums[i] == 1) q.push_back(i - (long long)q.size());
        }
        int c = q.size();
        vector<long long> prefix(c + 1, 0);
        for (int i = 0; i < c; i++) prefix[i + 1] = prefix[i] + q[i];
        long long best = LLONG_MAX;
        for (int lo = 0; lo + k <= c; lo++) {
            int hi = lo + k - 1;
            int mid = lo + k / 2;
            long long m = q[mid];
            long long cost = m * (mid - lo) - (prefix[mid] - prefix[lo])
                + (prefix[hi + 1] - prefix[mid + 1]) - m * (hi - mid);
            best = min(best, cost);
        }
        return (int)best;
    }
};
