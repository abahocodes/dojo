class Solution {
public:
    int peakIndexInMountain(vector<int>& arr) {
        int lo = 0, hi = (int)arr.size() - 1;
        while (lo < hi) {
            int mid = lo + (hi - lo) / 2;
            if (arr[mid] < arr[mid + 1]) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return lo;
    }
};
