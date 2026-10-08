class Solution {
    public int[] majorityElement(int[] nums) {
        int c1 = 0, c2 = 1, n1 = 0, n2 = 0;
        for (int x : nums) {
            if (x == c1) n1++;
            else if (x == c2) n2++;
            else if (n1 == 0) { c1 = x; n1 = 1; }
            else if (n2 == 0) { c2 = x; n2 = 1; }
            else { n1--; n2--; }
        }
        int f1 = 0, f2 = 0;
        for (int x : nums) {
            if (x == c1) f1++;
            else if (x == c2) f2++;
        }
        int limit = nums.length / 3;
        List<Integer> result = new ArrayList<>();
        if (f1 > limit) result.add(c1);
        if (f2 > limit) result.add(c2);
        return result.stream().mapToInt(Integer::intValue).toArray();
    }
}
