class Solution {
    public int evalRpn(String[] tokens) {
        Deque<Integer> stack = new ArrayDeque<>();
        for (String t : tokens) {
            switch (t) {
                case "+", "-", "*", "/" -> {
                    int b = stack.pop();
                    int a = stack.pop();
                    switch (t) {
                        case "+" -> stack.push(a + b);
                        case "-" -> stack.push(a - b);
                        case "*" -> stack.push(a * b);
                        default -> stack.push(a / b);
                    }
                }
                default -> stack.push(Integer.parseInt(t));
            }
        }
        return stack.pop();
    }
}
