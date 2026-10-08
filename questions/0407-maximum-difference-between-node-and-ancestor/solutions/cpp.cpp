class Solution {
public:
    int maxAncestorDiff(TreeNode* root) {
        int best = 0;
        vector<tuple<TreeNode*, int, int>> stack{{root, root->val, root->val}};
        while (!stack.empty()) {
            auto [node, lo, hi] = stack.back();
            stack.pop_back();
            lo = min(lo, node->val);
            hi = max(hi, node->val);
            best = max(best, hi - lo);
            if (node->left != nullptr) stack.emplace_back(node->left, lo, hi);
            if (node->right != nullptr) stack.emplace_back(node->right, lo, hi);
        }
        return best;
    }
};
