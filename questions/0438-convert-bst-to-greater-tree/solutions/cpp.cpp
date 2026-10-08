class Solution {
public:
    TreeNode* convertBst(TreeNode* root) {
        int running = 0;
        vector<TreeNode*> stack;
        TreeNode* node = root;
        while (!stack.empty() || node != nullptr) {
            while (node != nullptr) {
                stack.push_back(node);
                node = node->right;
            }
            node = stack.back();
            stack.pop_back();
            running += node->val;
            node->val = running;
            node = node->left;
        }
        return root;
    }
};
