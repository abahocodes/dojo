class Solution {
    public boolean checkValidString(String s) {
        // lo..hi is the range of possible counts of unmatched '(' so far.
        int lo = 0, hi = 0;
        for (char ch : s.toCharArray()) {
            if (ch == '(') {
                lo++;
                hi++;
            } else if (ch == ')') {
                lo--;
                hi--;
            } else { // '*' can be ')', nothing, or '('
                lo--;
                hi++;
            }
            if (hi < 0) return false;
            lo = Math.max(lo, 0);
        }
        return lo == 0;
    }
}
