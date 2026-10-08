class Solution {
    public int maxSatisfied(int[] customers, int[] grumpy, int minutes) {
        int base = 0, gain = 0, best = 0;
        for (int i = 0; i < customers.length; i++) {
            if (grumpy[i] == 0) base += customers[i];
            else gain += customers[i];
            if (i >= minutes && grumpy[i - minutes] == 1) gain -= customers[i - minutes];
            best = Math.max(best, gain);
        }
        return base + best;
    }
}
