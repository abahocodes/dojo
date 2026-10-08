class Solution {
public:
    int maxPathSum(TreeNode* root) {
        vector<TreeNode*> order;
        vector<TreeNode*> stack = {root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            order.push_back(node);
            if (node->left) stack.push_back(node->left);
            if (node->right) stack.push_back(node->right);
        }

        unordered_map<TreeNode*, int> gain;
        int best = root->val;
        for (int k = (int) order.size() - 1; k >= 0; k--) {
            TreeNode* node = order[k];
            int left = node->left ? max(gain[node->left], 0) : 0;
            int right = node->right ? max(gain[node->right], 0) : 0;
            best = max(best, node->val + left + right);
            gain[node] = node->val + max(left, right);
        }
        return best;
    }
};
