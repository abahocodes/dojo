class Solution {
    public int[] smallestRange(int[][] nums) {
        // Entries are {value, list, position}.
        PriorityQueue<int[]> heap = new PriorityQueue<>((x, y) -> Integer.compare(x[0], y[0]));
        int high = Integer.MIN_VALUE;
        for (int r = 0; r < nums.length; r++) {
            heap.add(new int[] {nums[r][0], r, 0});
            high = Math.max(high, nums[r][0]);
        }
        int[] best = {heap.peek()[0], high};
        while (true) {
            int[] e = heap.poll();
            int low = e[0], r = e[1], c = e[2];
            if (high - low < best[1] - best[0]) best = new int[] {low, high};
            if (c + 1 == nums[r].length) return best;
            int next = nums[r][c + 1];
            high = Math.max(high, next);
            heap.add(new int[] {next, r, c + 1});
        }
    }
}
