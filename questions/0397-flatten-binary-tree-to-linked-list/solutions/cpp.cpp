class Solution {
public:
    TreeNode* flatten(TreeNode* root) {
        TreeNode* node = root;
        while (node) {
            if (node->left) {
                // Splice the left subtree between node and its right subtree.
                TreeNode* tail = node->left;
                while (tail->right) tail = tail->right;
                tail->right = node->right;
                node->right = node->left;
                node->left = nullptr;
            }
            node = node->right;
        }
        return root;
    }
};
