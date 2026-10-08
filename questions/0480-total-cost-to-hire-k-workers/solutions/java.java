class Solution {
    public long totalCost(int[] costs, int k, int candidates) {
        Comparator<int[]> byCostThenIndex = (x, y) ->
            x[0] != y[0] ? Integer.compare(x[0], y[0]) : Integer.compare(x[1], y[1]);
        PriorityQueue<int[]> left = new PriorityQueue<>(byCostThenIndex);
        PriorityQueue<int[]> right = new PriorityQueue<>(byCostThenIndex);
        int i = 0, j = costs.length - 1;
        long total = 0;
        for (int round = 0; round < k; round++) {
            while (left.size() < candidates && i <= j) left.add(new int[] {costs[i], i++});
            while (right.size() < candidates && i <= j) right.add(new int[] {costs[j], j--});
            if (right.isEmpty() || (!left.isEmpty() && byCostThenIndex.compare(left.peek(), right.peek()) < 0)) {
                total += left.poll()[0];
            } else {
                total += right.poll()[0];
            }
        }
        return total;
    }
}
