class Solution {
    public String minRemoveToMakeValid(String s) {
        int n = s.length();
        boolean[] removed = new boolean[n];
        int[] stack = new int[n];
        int top = 0;
        for (int i = 0; i < n; i++) {
            char c = s.charAt(i);
            if (c == '(') {
                stack[top++] = i;
            } else if (c == ')') {
                if (top > 0) top--;
                else removed[i] = true;
            }
        }
        for (int k = 0; k < top; k++) removed[stack[k]] = true;
        StringBuilder sb = new StringBuilder(n);
        for (int i = 0; i < n; i++) {
            if (!removed[i]) sb.append(s.charAt(i));
        }
        return sb.toString();
    }
}
