class Solution {
    public int calculateNoParens(String s) {
        int total = 0, last = 0, num = 0;
        char op = '+';
        int n = s.length();
        for (int i = 0; i < n; i++) {
            char ch = s.charAt(i);
            boolean isDigit = ch >= '0' && ch <= '9';
            if (isDigit) num = num * 10 + (ch - '0');
            if ((!isDigit && ch != ' ') || i == n - 1) {
                switch (op) {
                    case '+': total += last; last = num; break;
                    case '-': total += last; last = -num; break;
                    case '*': last *= num; break;
                    default: last /= num; // Java truncates toward zero
                }
                op = ch;
                num = 0;
            }
        }
        return total + last;
    }
}
