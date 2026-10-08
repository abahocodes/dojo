class Solution {
    public int thirdMax(int[] nums) {
        // Long.MIN_VALUE marks an empty slot: no int can equal it.
        long first = Long.MIN_VALUE, second = Long.MIN_VALUE, third = Long.MIN_VALUE;
        for (int v : nums) {
            long x = v;
            if (x == first || x == second || x == third) continue;
            if (x > first) {
                third = second; second = first; first = x;
            } else if (x > second) {
                third = second; second = x;
            } else if (x > third) {
                third = x;
            }
        }
        return (int) (third == Long.MIN_VALUE ? first : third);
    }
}
