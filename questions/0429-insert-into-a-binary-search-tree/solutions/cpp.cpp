class Solution {
public:
    TreeNode* insertIntoBst(TreeNode* root, int val) {
        TreeNode* node = new TreeNode(val);
        if (!root) return node;
        TreeNode* cur = root;
        while (true) {
            TreeNode*& next = val < cur->val ? cur->left : cur->right;
            if (!next) {
                next = node;
                return root;
            }
            cur = next;
        }
    }
};
