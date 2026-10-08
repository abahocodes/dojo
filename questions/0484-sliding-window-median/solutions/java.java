class Solution {
    private PriorityQueue<Integer> low;  // max-heap: smaller half
    private PriorityQueue<Integer> high; // min-heap: larger half
    private Map<Integer, Integer> delayed; // value -> copies removed but still inside a heap
    private int lowSize;
    private int highSize;

    public double[] medianSlidingWindow(int[] nums, int k) {
        low = new PriorityQueue<>(Collections.reverseOrder());
        high = new PriorityQueue<>();
        delayed = new HashMap<>();
        lowSize = 0;
        highSize = 0;

        double[] result = new double[nums.length - k + 1];
        for (int i = 0; i < nums.length; i++) {
            add(nums[i]);
            if (i >= k) remove(nums[i - k]);
            if (i >= k - 1) {
                result[i - k + 1] = k % 2 == 1
                    ? (double) low.peek()
                    : ((double) low.peek() + (double) high.peek()) / 2.0;
            }
        }
        return result;
    }

    private void prune(PriorityQueue<Integer> heap) {
        while (!heap.isEmpty() && delayed.getOrDefault(heap.peek(), 0) > 0) {
            delayed.merge(heap.peek(), -1, Integer::sum);
            heap.poll();
        }
    }

    private void rebalance() {
        if (lowSize > highSize + 1) {
            high.offer(low.poll());
            lowSize--;
            highSize++;
            prune(low);
        } else if (lowSize < highSize) {
            low.offer(high.poll());
            lowSize++;
            highSize--;
            prune(high);
        }
    }

    private void add(int x) {
        if (low.isEmpty() || x <= low.peek()) {
            low.offer(x);
            lowSize++;
        } else {
            high.offer(x);
            highSize++;
        }
        rebalance();
    }

    private void remove(int x) {
        delayed.merge(x, 1, Integer::sum);
        if (x <= low.peek()) {
            lowSize--;
            if (x == low.peek()) prune(low);
        } else {
            highSize--;
            if (!high.isEmpty() && x == high.peek()) prune(high);
        }
        rebalance();
    }
}
