class Solution {
    public String removeKdigits(String num, int k) {
        char[] stack = new char[num.length()];
        int top = 0;
        for (int i = 0; i < num.length(); i++) {
            char d = num.charAt(i);
            while (k > 0 && top > 0 && stack[top - 1] > d) {
                top--;
                k--;
            }
            stack[top++] = d;
        }
        top -= k;
        int start = 0;
        while (start < top && stack[start] == '0') start++;
        return start == top ? "0" : new String(stack, start, top - start);
    }
}
