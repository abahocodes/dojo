class Solution {
    public int longestIncreasingPath(int[][] matrix) {
        int rows = matrix.length, cols = matrix[0].length;
        // cells by decreasing height, encoded as r * cols + c
        Integer[] order = new Integer[rows * cols];
        for (int i = 0; i < order.length; i++) order[i] = i;
        Arrays.sort(order, (a, b) -> Integer.compare(matrix[b / cols][b % cols], matrix[a / cols][a % cols]));

        int[][] best = new int[rows][cols];
        int[][] dirs = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};
        int answer = 1;
        for (int cell : order) {
            int r = cell / cols, c = cell % cols;
            int height = matrix[r][c];
            int length = 1;
            for (int[] d : dirs) {
                int nr = r + d[0], nc = c + d[1];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && matrix[nr][nc] > height) {
                    length = Math.max(length, best[nr][nc] + 1);
                }
            }
            best[r][c] = length;
            answer = Math.max(answer, length);
        }
        return answer;
    }
}
