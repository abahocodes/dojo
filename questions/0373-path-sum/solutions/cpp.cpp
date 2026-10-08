class Solution {
public:
    bool hasPathSum(TreeNode* root, int targetSum) {
        if (root == nullptr) return false;
        vector<pair<TreeNode*, int>> stack;
        stack.push_back({root, targetSum - root->val});
        while (!stack.empty()) {
            auto [node, remaining] = stack.back();
            stack.pop_back();
            if (node->left == nullptr && node->right == nullptr) {
                if (remaining == 0) return true;
                continue;
            }
            if (node->left) stack.push_back({node->left, remaining - node->left->val});
            if (node->right) stack.push_back({node->right, remaining - node->right->val});
        }
        return false;
    }
};
