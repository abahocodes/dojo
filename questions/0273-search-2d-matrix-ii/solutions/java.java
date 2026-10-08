class Solution {
    public boolean searchMatrixSorted(int[][] matrix, int target) {
        int rows = matrix.length;
        int r = 0, c = matrix[0].length - 1;
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
}
