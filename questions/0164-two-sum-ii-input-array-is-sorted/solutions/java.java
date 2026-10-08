class Solution {
    public int[] twoSumSorted(int[] numbers, int target) {
        int lo = 0, hi = numbers.length - 1;
        while (lo < hi) {
            int total = numbers[lo] + numbers[hi];
            if (total == target) return new int[] {lo + 1, hi + 1};
            if (total < target) lo++;
            else hi--;
        }
        return new int[0];
    }
}
