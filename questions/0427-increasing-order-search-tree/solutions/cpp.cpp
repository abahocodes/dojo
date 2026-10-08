class Solution {
public:
    TreeNode* increasingBst(TreeNode* root) {
        TreeNode dummy(0);
        TreeNode* tail = &dummy;
        vector<TreeNode*> stack;
        TreeNode* node = root;
        while (!stack.empty() || node) {
            while (node) {
                stack.push_back(node);
                node = node->left;
            }
            node = stack.back();
            stack.pop_back();
            node->left = nullptr;
            tail->right = node;
            tail = node;
            node = node->right;
        }
        return dummy.right;
    }
};
