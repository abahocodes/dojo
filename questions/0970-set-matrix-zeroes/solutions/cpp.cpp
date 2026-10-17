class Solution {
public:
    vector<vector<int>> setZeroes(vector<vector<int>>& matrix) {
        int rows = matrix.size(), cols = matrix[0].size();
        // Row 0 and column 0 double as markers; remember separately whether
        // they themselves need clearing.
        bool firstRowZero = false, firstColZero = false;
        for (int c = 0; c < cols; c++) if (matrix[0][c] == 0) firstRowZero = true;
        for (int r = 0; r < rows; r++) if (matrix[r][0] == 0) firstColZero = true;

        for (int r = 1; r < rows; r++) {
            for (int c = 1; c < cols; c++) {
                if (matrix[r][c] == 0) {
                    matrix[r][0] = 0;
                    matrix[0][c] = 0;
                }
            }
        }

        for (int r = 1; r < rows; r++) {
            for (int c = 1; c < cols; c++) {
                if (matrix[r][0] == 0 || matrix[0][c] == 0) matrix[r][c] = 0;
            }
        }

        if (firstRowZero) for (int c = 0; c < cols; c++) matrix[0][c] = 0;
        if (firstColZero) for (int r = 0; r < rows; r++) matrix[r][0] = 0;
        return matrix;
    }
};
