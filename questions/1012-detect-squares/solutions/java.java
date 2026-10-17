class DetectSquares {
    // Coordinates are 0..1000, so a grid of counts is small enough.
    private final int[][] cnt = new int[1001][1001];
    // x -> distinct ys stored in that column
    private final List<List<Integer>> ys = new ArrayList<>();

    public DetectSquares() {
        for (int i = 0; i <= 1000; i++) ys.add(new ArrayList<>());
    }

    public void add(int[] point) {
        int x = point[0], y = point[1];
        if (cnt[x][y]++ == 0) ys.get(x).add(y);
    }

    public int count(int[] point) {
        int qx = point[0], qy = point[1];
        long total = 0;
        // Every point in the query's column fixes the side length.
        for (int y : ys.get(qx)) {
            int side = y - qy;
            if (side == 0) continue;
            long c = cnt[qx][y];
            for (int px : new int[] {qx + side, qx - side}) {
                if (px < 0 || px > 1000) continue;
                total += c * cnt[px][qy] * cnt[px][y];
            }
        }
        return (int) total;
    }
}

// dojo calls this to run the tests; you don't need to change it.
class Solution {
    public int[] runDetectSquares(String[] ops, int[][] points) {
        DetectSquares ds = new DetectSquares();
        List<Integer> out = new ArrayList<>();
        for (int i = 0; i < ops.length; i++) {
            if (ops[i].equals("add")) ds.add(points[i]);
            else out.add(ds.count(points[i]));
        }
        int[] result = new int[out.size()];
        for (int i = 0; i < result.length; i++) result[i] = out.get(i);
        return result;
    }
}
