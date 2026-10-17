// 1002. Min Stack (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class MinStack {
    public MinStack() {

    }

    public void push(int val) {

    }

    public void pop() {

    }

    public int top() {
        return 0;
    }

    public int getMin() {
        return 0;
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
