class Solution {
public:
    bool leafSimilar(TreeNode* root1, TreeNode* root2) {
        return leaves(root1) == leaves(root2);
    }

private:
    vector<int> leaves(TreeNode* root) {
        vector<int> out;
        vector<TreeNode*> stack;
        if (root) stack.push_back(root);
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            if (!node->left && !node->right) {
                out.push_back(node->val);
                continue;
            }
            if (node->right) stack.push_back(node->right);
            if (node->left) stack.push_back(node->left);
        }
        return out;
    }
};
