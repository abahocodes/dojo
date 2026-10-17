class Solution {
    public boolean isHappy(int n) {
        // The digit-square sequence either reaches 1 or falls into a cycle.
        // Floyd's tortoise and hare finds out without remembering every value.
        int slow = n, fast = step(n);
        while (fast != 1 && slow != fast) {
            slow = step(slow);
            fast = step(step(fast));
        }
        return fast == 1;
    }

    private int step(int v) {
        int total = 0;
        while (v > 0) {
            int d = v % 10;
            total += d * d;
            v /= 10;
        }
        return total;
    }
}
