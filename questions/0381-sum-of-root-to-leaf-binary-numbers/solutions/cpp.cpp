class Solution {
public:
    int sumRootToLeaf(TreeNode* root) {
        int total = 0;
        vector<pair<TreeNode*, int>> stack;
        if (root) stack.push_back({root, 0});
        while (!stack.empty()) {
            auto [node, prefix] = stack.back();
            stack.pop_back();
            int value = prefix * 2 + node->val;
            if (!node->left && !node->right) {
                total += value;
                continue;
            }
            if (node->left) stack.push_back({node->left, value});
            if (node->right) stack.push_back({node->right, value});
        }
        return total;
    }
};
