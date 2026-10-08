class Solution {
public:
    int findTilt(TreeNode* root) {
        if (!root) return 0;
        vector<TreeNode*> order;
        vector<TreeNode*> stack = {root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            order.push_back(node);
            if (node->left) stack.push_back(node->left);
            if (node->right) stack.push_back(node->right);
        }
        unordered_map<TreeNode*, int> subtreeSum;
        int tilt = 0;
        for (int i = (int)order.size() - 1; i >= 0; i--) {
            TreeNode* node = order[i];
            int left = node->left ? subtreeSum[node->left] : 0;
            int right = node->right ? subtreeSum[node->right] : 0;
            tilt += abs(left - right);
            subtreeSum[node] = node->val + left + right;
        }
        return tilt;
    }
};
