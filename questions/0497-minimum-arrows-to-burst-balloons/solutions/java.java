class Solution {
    public int findMinArrowShots(int[][] points) {
        // Greedy: shoot each arrow at the right end of the balloon that ends first.
        // Integer.compare, not a[1] - b[1]: the subtraction can overflow.
        int[][] ordered = points.clone();
        Arrays.sort(ordered, (a, b) -> Integer.compare(a[1], b[1]));
        int arrows = 1;
        int pos = ordered[0][1];
        for (int[] p : ordered) {
            if (p[0] > pos) {
                arrows++;
                pos = p[1];
            }
        }
        return arrows;
    }
}
