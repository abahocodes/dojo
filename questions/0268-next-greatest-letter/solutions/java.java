class Solution {
    public String nextGreatestLetter(String[] letters, String target) {
        int lo = 0, hi = letters.length;
        while (lo < hi) {
            int mid = (lo + hi) >>> 1;
            if (letters[mid].compareTo(target) <= 0) {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        return letters[lo % letters.length];
    }
}
