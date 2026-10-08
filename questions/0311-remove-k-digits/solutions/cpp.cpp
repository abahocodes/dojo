class Solution {
public:
    string removeKdigits(string& num, int k) {
        string stack;
        for (char d : num) {
            while (k > 0 && !stack.empty() && stack.back() > d) {
                stack.pop_back();
                k--;
            }
            stack.push_back(d);
        }
        stack.resize(stack.size() - k);
        size_t start = 0;
        while (start < stack.size() && stack[start] == '0') start++;
        return start == stack.size() ? "0" : stack.substr(start);
    }
};
