class Solution {
public:
    int diameterOfBinaryTree(TreeNode* root) {
        if (!root) return 0;
        // visit nodes so that children come before their parent (reversed pre-order)
        vector<TreeNode*> order;
        vector<TreeNode*> stack{root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            order.push_back(node);
            if (node->left) stack.push_back(node->left);
            if (node->right) stack.push_back(node->right);
        }
        unordered_map<TreeNode*, int> height; // node -> number of nodes on its longest downward path
        int best = 0;
        for (auto it = order.rbegin(); it != order.rend(); ++it) {
            TreeNode* node = *it;
            int left = node->left ? height[node->left] : 0;
            int right = node->right ? height[node->right] : 0;
            best = max(best, left + right);
            height[node] = 1 + max(left, right);
        }
        return best;
    }
};
