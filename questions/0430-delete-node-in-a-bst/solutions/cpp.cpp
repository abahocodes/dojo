class Solution {
public:
    TreeNode* deleteNode(TreeNode* root, int key) {
        TreeNode* parent = nullptr;
        TreeNode* node = root;
        while (node && node->val != key) {
            parent = node;
            node = key < node->val ? node->left : node->right;
        }
        if (!node) return root;

        if (node->left && node->right) {
            TreeNode* succParent = node;
            TreeNode* succ = node->right;
            while (succ->left) {
                succParent = succ;
                succ = succ->left;
            }
            node->val = succ->val;
            if (succParent == node) succParent->right = succ->right;
            else succParent->left = succ->right;
            return root;
        }

        TreeNode* child = node->left ? node->left : node->right;
        if (!parent) return child;
        if (parent->left == node) parent->left = child;
        else parent->right = child;
        return root;
    }
};
