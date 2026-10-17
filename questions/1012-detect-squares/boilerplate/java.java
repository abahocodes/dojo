// 1012. Detect Squares (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class DetectSquares {
    public DetectSquares() {

    }

    public void add(int[] point) {

    }

    public int count(int[] point) {
        return 0;
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
