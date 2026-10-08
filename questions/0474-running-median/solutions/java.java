class Solution {
    public double[] runningMedian(int[] nums) {
        PriorityQueue<Integer> low = new PriorityQueue<>(Collections.reverseOrder()); // smaller half
        PriorityQueue<Integer> high = new PriorityQueue<>(); // larger half
        double[] medians = new double[nums.length];
        for (int i = 0; i < nums.length; i++) {
            int x = nums[i];
            if (low.isEmpty() || x <= low.peek()) low.add(x);
            else high.add(x);
            // Keep low.size() == high.size() or low.size() == high.size() + 1.
            if (low.size() > high.size() + 1) high.add(low.poll());
            else if (high.size() > low.size()) low.add(high.poll());
            medians[i] = low.size() > high.size()
                ? low.peek()
                : (low.peek() + (double) high.peek()) / 2.0;
        }
        return medians;
    }
}
