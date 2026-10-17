// 1002. Min Stack (medium)
// /show to reread the problem · /test runs visible tests · /submit when done

class MinStack {
public:
    MinStack() {

    }

    void push(int val) {

    }

    void pop() {

    }

    int top() {
        return 0;
    }

    int getMin() {
        return 0;
    }
};

class Solution {
public:
    // dojo calls this to run the tests; you don't need to change it.
    vector<int> minStack(vector<string>& ops, vector<int>& vals) {
        MinStack stack;
        vector<int> out;
        for (size_t i = 0; i < ops.size(); i++) {
            if (ops[i] == "push") stack.push(vals[i]);
            else if (ops[i] == "pop") stack.pop();
            else if (ops[i] == "top") out.push_back(stack.top());
            else out.push_back(stack.getMin());
        }
        return out;
    }
};
