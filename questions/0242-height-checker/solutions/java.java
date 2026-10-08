class Solution {
    public int heightChecker(int[] heights) {
        int[] count = new int[101];
        for (int h : heights) count[h]++;
        int mismatches = 0, expected = 1;
        for (int h : heights) {
            while (count[expected] == 0) expected++;
            if (h != expected) mismatches++;
            count[expected]--;
        }
        return mismatches;
    }
}
