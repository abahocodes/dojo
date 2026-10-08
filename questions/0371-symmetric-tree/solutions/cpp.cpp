class Solution {
public:
    bool isSymmetric(TreeNode* root) {
        vector<pair<TreeNode*, TreeNode*>> stack = {{root->left, root->right}};
        while (!stack.empty()) {
            auto [a, b] = stack.back();
            stack.pop_back();
            if (a == nullptr && b == nullptr) continue;
            if (a == nullptr || b == nullptr || a->val != b->val) return false;
            stack.push_back({a->left, b->right});
            stack.push_back({a->right, b->left});
        }
        return true;
    }
};
