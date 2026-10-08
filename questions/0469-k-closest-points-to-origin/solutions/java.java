class Solution {
    public int[][] kClosest(int[][] points, int k) {
        // Max-heap of point indices by squared distance, holding the k closest so far.
        PriorityQueue<int[]> heap = new PriorityQueue<>((a, b) -> Integer.compare(b[0], a[0]));
        for (int i = 0; i < points.length; i++) {
            int x = points[i][0], y = points[i][1];
            int d = x * x + y * y;
            if (heap.size() < k) {
                heap.add(new int[] {d, i});
            } else if (d < heap.peek()[0]) {
                heap.poll();
                heap.add(new int[] {d, i});
            }
        }
        int[][] result = new int[heap.size()][];
        int j = 0;
        for (int[] entry : heap) result[j++] = points[entry[1]];
        return result;
    }
}
