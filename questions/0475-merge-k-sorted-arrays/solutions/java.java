class Solution {
    public int[] mergeKSortedArrays(int[][] arrays) {
        // Min-heap of {value, array index, position}, ordered by value.
        PriorityQueue<int[]> heap = new PriorityQueue<>((x, y) -> Integer.compare(x[0], y[0]));
        int total = 0;
        for (int a = 0; a < arrays.length; a++) {
            total += arrays[a].length;
            if (arrays[a].length > 0) heap.add(new int[] {arrays[a][0], a, 0});
        }
        int[] merged = new int[total];
        int out = 0;
        while (!heap.isEmpty()) {
            int[] top = heap.poll();
            merged[out++] = top[0];
            int a = top[1], p = top[2];
            if (p + 1 < arrays[a].length) heap.add(new int[] {arrays[a][p + 1], a, p + 1});
        }
        return merged;
    }
}
