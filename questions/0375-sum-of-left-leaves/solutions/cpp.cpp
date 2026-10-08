class Solution {
public:
    int sumOfLeftLeaves(TreeNode* root) {
        int total = 0;
        vector<TreeNode*> stack = {root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            TreeNode* left = node->left;
            if (left) {
                if (left->left == nullptr && left->right == nullptr) total += left->val;
                else stack.push_back(left);
            }
            if (node->right) stack.push_back(node->right);
        }
        return total;
    }
};
