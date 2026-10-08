class Solution {
public:
    string makeGood(string& s) {
        string stack;
        for (char ch : s) {
            // 'a' and 'A' differ by exactly 32 in ASCII.
            if (!stack.empty() && abs(stack.back() - ch) == 32) stack.pop_back();
            else stack.push_back(ch);
        }
        return stack;
    }
};
