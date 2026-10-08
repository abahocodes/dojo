class Solution {
public:
    TreeNode* mergeTrees(TreeNode* root1, TreeNode* root2) {
        if (root1 == nullptr) return root2;
        vector<pair<TreeNode*, TreeNode*>> stack;
        stack.push_back({root1, root2});
        while (!stack.empty()) {
            auto [a, b] = stack.back();
            stack.pop_back();
            if (b == nullptr) continue;
            a->val += b->val;
            if (a->left == nullptr) a->left = b->left;
            else stack.push_back({a->left, b->left});
            if (a->right == nullptr) a->right = b->right;
            else stack.push_back({a->right, b->right});
        }
        return root1;
    }
};
