class Solution {
public:
    double findMedianSortedArrays(vector<int>& nums1, vector<int>& nums2) {
        if (nums1.size() > nums2.size()) {
            return findMedianSortedArrays(nums2, nums1);
        }
        int m = nums1.size(), n = nums2.size();
        int half = (m + n + 1) / 2;
        const double inf = numeric_limits<double>::infinity();
        int lo = 0, hi = m;
        while (lo <= hi) {
            int i = (lo + hi) / 2;
            int j = half - i;
            double left1 = i > 0 ? nums1[i - 1] : -inf;
            double right1 = i < m ? nums1[i] : inf;
            double left2 = j > 0 ? nums2[j - 1] : -inf;
            double right2 = j < n ? nums2[j] : inf;
            if (left1 > right2) {
                hi = i - 1;
            } else if (left2 > right1) {
                lo = i + 1;
            } else {
                if ((m + n) % 2 == 1) {
                    return max(left1, left2);
                }
                return (max(left1, left2) + min(right1, right2)) / 2;
            }
        }
        return 0.0;
    }
};
