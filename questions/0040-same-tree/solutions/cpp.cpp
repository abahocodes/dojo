class Solution {
public:
    bool isSameTree(TreeNode* p, TreeNode* q) {
        vector<pair<TreeNode*, TreeNode*>> stack = {{p, q}};
        while (!stack.empty()) {
            auto [a, b] = stack.back();
            stack.pop_back();
            if (!a && !b) continue;
            if (!a || !b || a->val != b->val) return false;
            stack.push_back({a->left, b->left});
            stack.push_back({a->right, b->right});
        }
        return true;
    }
};
