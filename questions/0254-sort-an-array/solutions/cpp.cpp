class Solution {
public:
    vector<int> sortArray(vector<int>& nums) {
        int n = nums.size();
        vector<int> a = nums, buf(n);
        for (int width = 1; width < n; width *= 2) {
            for (int lo = 0; lo < n; lo += 2 * width) {
                int mid = min(lo + width, n);
                int hi = min(lo + 2 * width, n);
                int i = lo, j = mid, k = lo;
                while (i < mid && j < hi) buf[k++] = a[i] <= a[j] ? a[i++] : a[j++];
                while (i < mid) buf[k++] = a[i++];
                while (j < hi) buf[k++] = a[j++];
            }
            swap(a, buf);
        }
        return a;
    }
};
