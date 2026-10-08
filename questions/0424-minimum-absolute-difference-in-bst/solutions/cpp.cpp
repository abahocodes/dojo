class Solution {
public:
    int getMinimumDifference(TreeNode* root) {
        int best = INT_MAX;
        bool hasPrev = false;
        int prev = 0;
        vector<TreeNode*> stack;
        TreeNode* node = root;
        while (!stack.empty() || node) {
            while (node) {
                stack.push_back(node);
                node = node->left;
            }
            node = stack.back();
            stack.pop_back();
            if (hasPrev) best = min(best, node->val - prev);
            prev = node->val;
            hasPrev = true;
            node = node->right;
        }
        return best;
    }
};
