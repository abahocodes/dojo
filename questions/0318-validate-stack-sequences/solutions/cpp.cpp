class Solution {
public:
    bool validateStackSequences(vector<int>& pushed, vector<int>& popped) {
        vector<int> stack;
        size_t j = 0;
        for (int x : pushed) {
            stack.push_back(x);
            while (!stack.empty() && stack.back() == popped[j]) {
                stack.pop_back();
                j++;
            }
        }
        return stack.empty();
    }
};
