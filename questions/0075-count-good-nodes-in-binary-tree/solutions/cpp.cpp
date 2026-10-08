class Solution {
public:
    int goodNodes(TreeNode* root) {
        if (!root) return 0;
        int count = 0;
        // (node, largest value strictly above it on the path)
        vector<pair<TreeNode*, int>> stack{{root, root->val}};
        while (!stack.empty()) {
            auto [node, best] = stack.back();
            stack.pop_back();
            if (node->val >= best) {
                count++;
                best = node->val;
            }
            if (node->left) stack.push_back({node->left, best});
            if (node->right) stack.push_back({node->right, best});
        }
        return count;
    }
};
