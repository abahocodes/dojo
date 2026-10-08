class Solution {
    public boolean isValid(String s) {
        Deque<Character> stack = new ArrayDeque<>();
        for (char ch : s.toCharArray()) {
            char open = ch == ')' ? '(' : ch == ']' ? '[' : ch == '}' ? '{' : 0;
            if (open != 0) {
                if (stack.isEmpty() || stack.peek() != open) return false;
                stack.pop();
            } else {
                stack.push(ch);
            }
        }
        return stack.isEmpty();
    }
}
