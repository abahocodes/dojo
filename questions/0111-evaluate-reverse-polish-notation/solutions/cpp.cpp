class Solution {
public:
    int evalRpn(vector<string>& tokens) {
        vector<int> stack;
        for (const string& t : tokens) {
            if (t == "+" || t == "-" || t == "*" || t == "/") {
                int b = stack.back();
                stack.pop_back();
                int a = stack.back();
                stack.pop_back();
                if (t == "+") stack.push_back(a + b);
                else if (t == "-") stack.push_back(a - b);
                else if (t == "*") stack.push_back(a * b);
                else stack.push_back(a / b);
            } else {
                stack.push_back(stoi(t));
            }
        }
        return stack[0];
    }
};
