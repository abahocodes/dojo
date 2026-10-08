class Solution {
    public String makeGood(String s) {
        StringBuilder stack = new StringBuilder();
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            int top = stack.length() - 1;
            // 'a' and 'A' differ by exactly 32 in ASCII.
            if (top >= 0 && Math.abs(stack.charAt(top) - ch) == 32) stack.deleteCharAt(top);
            else stack.append(ch);
        }
        return stack.toString();
    }
}
