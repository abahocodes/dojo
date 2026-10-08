class Solution {
public:
    bool isValidBst(TreeNode* root) {
        vector<TreeNode*> stack;
        bool hasPrev = false;
        int prev = 0;
        TreeNode* node = root;
        while (!stack.empty() || node) {
            while (node) {
                stack.push_back(node);
                node = node->left;
            }
            node = stack.back();
            stack.pop_back();
            if (hasPrev && node->val <= prev) return false;
            hasPrev = true;
            prev = node->val;
            node = node->right;
        }
        return true;
    }
};
