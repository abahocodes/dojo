class Solution {
public:
    int countNodes(TreeNode* root) {
        int count = 0;
        TreeNode* node = root;
        while (node) {
            int lh = leftDepth(node->left);
            int rh = leftDepth(node->right);
            if (lh == rh) {
                count += 1 << lh;
                node = node->right;
            } else {
                count += 1 << rh;
                node = node->left;
            }
        }
        return count;
    }

private:
    int leftDepth(TreeNode* node) {
        int depth = 0;
        while (node) {
            depth++;
            node = node->left;
        }
        return depth;
    }
};
