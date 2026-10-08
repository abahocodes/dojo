class Solution {
public:
    bool isBalanced(TreeNode* root) {
        if (!root) return true;
        // reversed pre-order puts every child before its parent
        vector<TreeNode*> order;
        vector<TreeNode*> stack{root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            order.push_back(node);
            if (node->left) stack.push_back(node->left);
            if (node->right) stack.push_back(node->right);
        }
        unordered_map<TreeNode*, int> height;
        for (auto it = order.rbegin(); it != order.rend(); ++it) {
            TreeNode* node = *it;
            int left = node->left ? height[node->left] : 0;
            int right = node->right ? height[node->right] : 0;
            if (abs(left - right) > 1) return false;
            height[node] = 1 + max(left, right);
        }
        return true;
    }
};
