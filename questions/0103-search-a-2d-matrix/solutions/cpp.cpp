class Solution {
public:
    bool searchMatrix(vector<vector<int>>& matrix, int target) {
        int cols = matrix[0].size();
        int lo = 0, hi = (int)matrix.size() * cols - 1;
        while (lo <= hi) {
            int mid = lo + (hi - lo) / 2;
            int value = matrix[mid / cols][mid % cols];
            if (value == target) return true;
            if (value < target) lo = mid + 1;
            else hi = mid - 1;
        }
        return false;
    }
};
