class Solution {
    public int longestValidParentheses(String s) {
        int n = s.length();
        int[] stack = new int[n + 1];
        int top = 0;
        stack[top++] = -1; // bottom entry is the last unmatched position
        int best = 0;
        for (int i = 0; i < n; i++) {
            if (s.charAt(i) == '(') {
                stack[top++] = i;
            } else {
                top--;
                if (top == 0) {
                    stack[top++] = i;
                } else {
                    best = Math.max(best, i - stack[top - 1]);
                }
            }
        }
        return best;
    }
}
