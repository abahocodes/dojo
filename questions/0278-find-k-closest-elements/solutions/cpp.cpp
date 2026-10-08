class Solution {
public:
    vector<int> findClosestElements(vector<int>& arr, int k, int x) {
        // Binary search for the left edge of the best window arr[left .. left + k - 1].
        int lo = 0, hi = (int)arr.size() - k;
        while (lo < hi) {
            int mid = (lo + hi) / 2;
            if (x - arr[mid] > arr[mid + k] - x) lo = mid + 1;
            else hi = mid;
        }
        return vector<int>(arr.begin() + lo, arr.begin() + lo + k);
    }
};
