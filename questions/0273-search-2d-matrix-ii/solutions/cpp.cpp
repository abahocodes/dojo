class Solution {
public:
    bool searchMatrixSorted(vector<vector<int>>& matrix, int target) {
        int rows = (int)matrix.size();
        int r = 0, c = (int)matrix[0].size() - 1;
        while (r < rows && c >= 0) {
            int value = matrix[r][c];
            if (value == target) {
                return true;
            }
            if (value > target) {
                c--;
            } else {
                r++;
            }
        }
        return false;
    }
};
