class Solution {
public:
    bool isValid(string& s) {
        string stack;
        for (char ch : s) {
            char open = ch == ')' ? '(' : ch == ']' ? '[' : ch == '}' ? '{' : 0;
            if (open) {
                if (stack.empty() || stack.back() != open) return false;
                stack.pop_back();
            } else {
                stack.push_back(ch);
            }
        }
        return stack.empty();
    }
};
