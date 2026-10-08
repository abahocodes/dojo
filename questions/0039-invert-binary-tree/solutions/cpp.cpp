class Solution {
public:
    TreeNode* invertTree(TreeNode* root) {
        vector<TreeNode*> stack = {root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            if (node) {
                swap(node->left, node->right);
                stack.push_back(node->left);
                stack.push_back(node->right);
            }
        }
        return root;
    }
};
