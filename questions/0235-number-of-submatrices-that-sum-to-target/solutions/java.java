class Solution {
    public int numSubmatrixSumTarget(int[][] matrix, int target) {
        int rows = matrix.length;
        int cols = matrix[0].length;
        int count = 0;
        Map<Integer, Integer> seen = new HashMap<>();
        for (int top = 0; top < rows; top++) {
            int[] col = new int[cols];
            for (int bottom = top; bottom < rows; bottom++) {
                for (int c = 0; c < cols; c++) col[c] += matrix[bottom][c];
                seen.clear();
                seen.put(0, 1);
                int s = 0;
                for (int v : col) {
                    s += v;
                    count += seen.getOrDefault(s - target, 0);
                    seen.merge(s, 1, Integer::sum);
                }
            }
        }
        return count;
    }
}
