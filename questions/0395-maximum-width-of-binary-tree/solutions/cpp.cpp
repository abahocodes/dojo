class Solution {
public:
    int widthOfBinaryTree(TreeNode* root) {
        long long best = 0;
        vector<pair<TreeNode*, long long>> level{{root, 0}};
        while (!level.empty()) {
            long long base = level.front().second;
            best = max(best, level.back().second - base + 1);
            vector<pair<TreeNode*, long long>> next;
            for (auto& [node, p] : level) {
                long long pos = p - base;
                if (node->left) next.push_back({node->left, 2 * pos});
                if (node->right) next.push_back({node->right, 2 * pos + 1});
            }
            level = move(next);
        }
        return (int)best;
    }
};
