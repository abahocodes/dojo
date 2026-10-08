class Solution {
public:
    vector<int> mergeSorted(vector<int>& nums1, vector<int>& nums2) {
        int m = nums1.size(), n = nums2.size();
        vector<int> out(nums1);
        out.resize(m + n);
        int i = m - 1, j = n - 1, w = m + n - 1;
        while (j >= 0) {
            if (i >= 0 && out[i] > nums2[j]) {
                out[w--] = out[i--];
            } else {
                out[w--] = nums2[j--];
            }
        }
        return out;
    }
};
