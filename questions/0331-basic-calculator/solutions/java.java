class Solution {
    public int calculate(String s) {
        long result = 0, num = 0;
        int sign = 1;
        Deque<Long> stack = new ArrayDeque<>();
        for (int i = 0; i < s.length(); i++) {
            char ch = s.charAt(i);
            if (ch >= '0' && ch <= '9') {
                num = num * 10 + (ch - '0');
            } else if (ch == '+' || ch == '-') {
                result += sign * num;
                num = 0;
                sign = ch == '+' ? 1 : -1;
            } else if (ch == '(') {
                stack.push(result);
                stack.push((long) sign);
                result = 0;
                sign = 1;
            } else if (ch == ')') {
                result += sign * num;
                num = 0;
                long savedSign = stack.pop();
                long savedResult = stack.pop();
                result = savedResult + savedSign * result;
            }
        }
        return (int) (result + sign * num);
    }
}
