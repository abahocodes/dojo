class Solution {
    public int longestCommonPrefixNumbers(int[] arr1, int[] arr2) {
        Set<Integer> prefixes = new HashSet<>();
        for (int x : arr1) {
            while (x > 0 && prefixes.add(x)) x /= 10;
        }
        int best = 0;
        for (int y : arr2) {
            while (y > 0 && !prefixes.contains(y)) y /= 10;
            int digits = 0;
            for (int t = y; t > 0; t /= 10) digits++;
            best = Math.max(best, digits);
        }
        return best;
    }
}
