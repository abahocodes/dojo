class Solution {
public:
    int rangeSumBst(TreeNode* root, int low, int high) {
        int total = 0;
        vector<TreeNode*> stack;
        if (root) stack.push_back(root);
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            if (node->val < low) {
                if (node->right) stack.push_back(node->right);
            } else if (node->val > high) {
                if (node->left) stack.push_back(node->left);
            } else {
                total += node->val;
                if (node->left) stack.push_back(node->left);
                if (node->right) stack.push_back(node->right);
            }
        }
        return total;
    }
};
