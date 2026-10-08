class Solution {
public:
    TreeNode* constructMaximumBinaryTree(vector<int>& nums) {
        vector<TreeNode*> stack;
        for (int x : nums) {
            TreeNode* node = new TreeNode(x);
            TreeNode* last = nullptr;
            while (!stack.empty() && stack.back()->val < x) {
                last = stack.back();
                stack.pop_back();
            }
            node->left = last;
            if (!stack.empty()) stack.back()->right = node;
            stack.push_back(node);
        }
        return stack.front();
    }
};
