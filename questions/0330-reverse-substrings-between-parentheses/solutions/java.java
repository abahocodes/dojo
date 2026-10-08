class Solution {
    public String reverseParentheses(String s) {
        int n = s.length();
        int[] partner = new int[n];
        int[] opens = new int[n];
        int top = -1;
        for (int i = 0; i < n; i++) {
            char ch = s.charAt(i);
            if (ch == '(') {
                opens[++top] = i;
            } else if (ch == ')') {
                int j = opens[top--];
                partner[i] = j;
                partner[j] = i;
            }
        }
        StringBuilder out = new StringBuilder();
        int step = 1;
        for (int i = 0; i < n; i += step) {
            char ch = s.charAt(i);
            if (ch == '(' || ch == ')') {
                i = partner[i];
                step = -step;
            } else {
                out.append(ch);
            }
        }
        return out.toString();
    }
}
