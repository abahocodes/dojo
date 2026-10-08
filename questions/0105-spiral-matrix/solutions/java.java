class Solution {
    public int[] spiralOrder(int[][] matrix) {
        int top = 0, bottom = matrix.length - 1;
        int left = 0, right = matrix[0].length - 1;
        int[] out = new int[matrix.length * matrix[0].length];
        int k = 0;
        while (top <= bottom && left <= right) {
            for (int c = left; c <= right; c++) out[k++] = matrix[top][c];
            top++;
            for (int r = top; r <= bottom; r++) out[k++] = matrix[r][right];
            right--;
            if (top <= bottom) {
                for (int c = right; c >= left; c--) out[k++] = matrix[bottom][c];
                bottom--;
            }
            if (left <= right) {
                for (int r = bottom; r >= top; r--) out[k++] = matrix[r][left];
                left++;
            }
        }
        return out;
    }
}
