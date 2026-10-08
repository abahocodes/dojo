class Solution {
public:
    int smallestDistancePair(vector<int>& nums, int k) {
        vector<int> sorted(nums);
        sort(sorted.begin(), sorted.end());
        int lo = 0, hi = sorted.back() - sorted.front();
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (pairsWithin(sorted, mid) >= k) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

private:
    long long pairsWithin(const vector<int>& sorted, int limit) {
        long long count = 0;
        int left = 0;
        for (int right = 0; right < (int)sorted.size(); right++) {
            while (sorted[right] - sorted[left] > limit) left++;
            count += right - left;
        }
        return count;
    }
};
