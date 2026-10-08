class Solution {
public:
    bool verifyPreorder(vector<int>& preorder) {
        int low = INT_MIN;
        vector<int> stack;
        for (int x : preorder) {
            if (x < low) return false;
            while (!stack.empty() && stack.back() < x) {
                low = stack.back();
                stack.pop_back();
            }
            stack.push_back(x);
        }
        return true;
    }
};
