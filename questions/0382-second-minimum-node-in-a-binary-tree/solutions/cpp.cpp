class Solution {
public:
    int findSecondMinimumValue(TreeNode* root) {
        int smallest = root->val;
        int best = -1;
        vector<TreeNode*> stack = {root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            if (node->val > smallest) {
                if (best == -1 || node->val < best) best = node->val;
                continue;
            }
            if (node->left) {
                stack.push_back(node->left);
                stack.push_back(node->right);
            }
        }
        return best;
    }
};
