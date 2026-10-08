class Solution {
public:
    long long kthSmallestProduct(vector<int>& nums1, vector<int>& nums2, long long k) {
        long long a0 = nums1.front(), a1 = nums1.back();
        long long b0 = nums2.front(), b1 = nums2.back();
        long long lo = min({a0 * b0, a0 * b1, a1 * b0, a1 * b1});
        long long hi = max({a0 * b0, a0 * b1, a1 * b0, a1 * b1});
        while (lo < hi) {
            long long mid = lo + (hi - lo) / 2;
            if (countAtMost(nums1, nums2, mid) >= k) hi = mid;
            else lo = mid + 1;
        }
        return lo;
    }

private:
    long long countAtMost(const vector<int>& nums1, const vector<int>& nums2, long long x) {
        long long total = 0;
        for (int a : nums1) total += countFor(a, nums2, x);
        return total;
    }

    // Number of j with a * nums2[j] <= x.
    int countFor(long long a, const vector<int>& nums2, long long x) {
        int n2 = (int)nums2.size();
        if (a == 0) return x >= 0 ? n2 : 0;
        int lo = 0, hi = n2;
        if (a > 0) {
            while (lo < hi) {
                int mid = lo + (hi - lo) / 2;
                if (a * nums2[mid] <= x) lo = mid + 1;
                else hi = mid;
            }
            return lo;
        }
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (a * nums2[mid] <= x) hi = mid;
            else lo = mid + 1;
        }
        return n2 - lo;
    }
};
