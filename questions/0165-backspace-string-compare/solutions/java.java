class Solution {
    public boolean backspaceCompare(String s, String t) {
        int i = s.length() - 1, j = t.length() - 1;
        while (true) {
            i = prevChar(s, i);
            j = prevChar(t, j);
            if (i < 0 || j < 0) return i < 0 && j < 0;
            if (s.charAt(i) != t.charAt(j)) return false;
            i--;
            j--;
        }
    }

    // Index of the next surviving character at or before i, or -1.
    private int prevChar(String text, int i) {
        int skip = 0;
        while (i >= 0) {
            if (text.charAt(i) == '#') skip++;
            else if (skip > 0) skip--;
            else return i;
            i--;
        }
        return -1;
    }
}
