class Solution {
    public int maximalSquare(String[][] matrix) {
        int cols = matrix[0].length;
        int[] side = new int[cols + 1];
        int best = 0;
        for (String[] row : matrix) {
            int prevDiag = 0;
            for (int c = 1; c <= cols; c++) {
                int above = side[c];
                if (row[c - 1].equals("1")) {
                    side[c] = 1 + Math.min(above, Math.min(side[c - 1], prevDiag));
                    best = Math.max(best, side[c]);
                } else {
                    side[c] = 0;
                }
                prevDiag = above;
            }
        }
        return best * best;
    }
}
