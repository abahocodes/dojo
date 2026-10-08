class Solution {
public:
    bool isUnivalTree(TreeNode* root) {
        int value = root->val;
        vector<TreeNode*> stack = {root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            if (node->val != value) return false;
            if (node->left) stack.push_back(node->left);
            if (node->right) stack.push_back(node->right);
        }
        return true;
    }
};
