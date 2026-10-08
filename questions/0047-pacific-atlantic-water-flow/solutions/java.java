class Solution {
    private static final int[][] DIRS = {{1, 0}, {-1, 0}, {0, 1}, {0, -1}};

    public int[][] pacificAtlantic(int[][] heights) {
        int rows = heights.length, cols = heights[0].length;
        List<int[]> pacificStarts = new ArrayList<>();
        List<int[]> atlanticStarts = new ArrayList<>();
        for (int c = 0; c < cols; c++) {
            pacificStarts.add(new int[]{0, c});
            atlanticStarts.add(new int[]{rows - 1, c});
        }
        for (int r = 0; r < rows; r++) {
            pacificStarts.add(new int[]{r, 0});
            atlanticStarts.add(new int[]{r, cols - 1});
        }
        boolean[][] pacific = reachable(heights, pacificStarts);
        boolean[][] atlantic = reachable(heights, atlanticStarts);
        List<int[]> result = new ArrayList<>();
        for (int r = 0; r < rows; r++) {
            for (int c = 0; c < cols; c++) {
                if (pacific[r][c] && atlantic[r][c]) result.add(new int[]{r, c});
            }
        }
        return result.toArray(new int[0][]);
    }

    // Walk uphill from the ocean: water can flow from each reached cell
    // down to the ocean. Iterative DFS keeps large grids off the call stack.
    private boolean[][] reachable(int[][] heights, List<int[]> starts) {
        int rows = heights.length, cols = heights[0].length;
        boolean[][] seen = new boolean[rows][cols];
        Deque<int[]> stack = new ArrayDeque<>();
        for (int[] s : starts) {
            if (!seen[s[0]][s[1]]) {
                seen[s[0]][s[1]] = true;
                stack.push(s);
            }
        }
        while (!stack.isEmpty()) {
            int[] cell = stack.pop();
            int r = cell[0], c = cell[1];
            for (int[] d : DIRS) {
                int nr = r + d[0], nc = c + d[1];
                if (nr >= 0 && nr < rows && nc >= 0 && nc < cols && !seen[nr][nc]
                        && heights[nr][nc] >= heights[r][c]) {
                    seen[nr][nc] = true;
                    stack.push(new int[]{nr, nc});
                }
            }
        }
        return seen;
    }
}
