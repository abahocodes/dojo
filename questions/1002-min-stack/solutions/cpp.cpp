class MinStack {
    // Each entry is {value, minimum of this entry and everything below it}.
    vector<pair<int, int>> items;

public:
    MinStack() {}

    void push(int val) {
        int low = items.empty() ? val : min(val, items.back().second);
        items.push_back({val, low});
    }

    void pop() {
        items.pop_back();
    }

    int top() {
        return items.back().first;
    }

    int getMin() {
        return items.back().second;
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
