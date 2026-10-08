class Solution {
    public int maxDepthParens(String s) {
        int depth = 0, best = 0;
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            if (ch == '(') best = Math.max(best, ++depth);
            else if (ch == ')') depth--;
        }
        return best;
    }
}
