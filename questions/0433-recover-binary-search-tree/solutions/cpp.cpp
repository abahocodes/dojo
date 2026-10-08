class Solution {
public:
    TreeNode* recoverTree(TreeNode* root) {
        TreeNode *first = nullptr, *second = nullptr, *prev = nullptr;
        vector<TreeNode*> stack;
        TreeNode* node = root;
        while (!stack.empty() || node) {
            while (node) {
                stack.push_back(node);
                node = node->left;
            }
            node = stack.back();
            stack.pop_back();
            if (prev && prev->val > node->val) {
                if (!first) first = prev;
                second = node;
            }
            prev = node;
            node = node->right;
        }
        swap(first->val, second->val);
        return root;
    }
};
