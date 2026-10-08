class Solution {
public:
    TreeNode* subtreeWithAllDeepest(TreeNode* root) {
        return dfs(root).second;
    }

private:
    // (height of the subtree, root of the answer inside it)
    pair<int, TreeNode*> dfs(TreeNode* node) {
        if (node == nullptr) return {0, nullptr};
        auto [lh, la] = dfs(node->left);
        auto [rh, ra] = dfs(node->right);
        if (lh > rh) return {lh + 1, la};
        if (rh > lh) return {rh + 1, ra};
        return {lh + 1, node};
    }
};
