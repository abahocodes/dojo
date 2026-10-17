class MinStack {
    // Parallel stacks: values, and the minimum of each entry and everything below it.
    private final ArrayDeque<Integer> values = new ArrayDeque<>();
    private final ArrayDeque<Integer> mins = new ArrayDeque<>();

    public MinStack() {}

    public void push(int val) {
        values.push(val);
        mins.push(mins.isEmpty() ? val : Math.min(val, mins.peek()));
    }

    public void pop() {
        values.pop();
        mins.pop();
    }

    public int top() {
        return values.peek();
    }

    public int getMin() {
        return mins.peek();
    }
}

class Solution {
    // dojo calls this to run the tests; you don't need to change it.
    public int[] minStack(String[] ops, int[] vals) {
        MinStack stack = new MinStack();
        List<Integer> out = new ArrayList<>();
        for (int i = 0; i < ops.length; i++) {
            switch (ops[i]) {
                case "push": stack.push(vals[i]); break;
                case "pop": stack.pop(); break;
                case "top": out.add(stack.top()); break;
                default: out.add(stack.getMin());
            }
        }
        int[] result = new int[out.size()];
        for (int i = 0; i < result.length; i++) result[i] = out.get(i);
        return result;
    }
}
