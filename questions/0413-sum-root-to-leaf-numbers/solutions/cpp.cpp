class Solution {
public:
    int sumNumbers(TreeNode* root) {
        int total = 0;
        vector<pair<TreeNode*, int>> stack{{root, 0}};
        while (!stack.empty()) {
            auto [node, prefix] = stack.back();
            stack.pop_back();
            int value = prefix * 10 + node->val;
            if (node->left == nullptr && node->right == nullptr) {
                total += value;
                continue;
            }
            if (node->left) stack.push_back({node->left, value});
            if (node->right) stack.push_back({node->right, value});
        }
        return total;
    }
};
