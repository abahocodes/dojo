class Solution {
public:
    TreeNode* trimBst(TreeNode* root, int low, int high) {
        while (root && (root->val < low || root->val > high)) {
            root = root->val < low ? root->right : root->left;
        }
        if (!root) return nullptr;

        TreeNode* node = root;
        while (node->left) {
            if (node->left->val < low) node->left = node->left->right;
            else node = node->left;
        }

        node = root;
        while (node->right) {
            if (node->right->val > high) node->right = node->right->left;
            else node = node->right;
        }
        return root;
    }
};
