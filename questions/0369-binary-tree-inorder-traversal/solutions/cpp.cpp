class Solution {
public:
    vector<int> inorderTraversal(TreeNode* root) {
        vector<int> result;
        vector<TreeNode*> stack;
        TreeNode* cur = root;
        while (cur != nullptr || !stack.empty()) {
            while (cur != nullptr) {
                stack.push_back(cur);
                cur = cur->left;
            }
            cur = stack.back();
            stack.pop_back();
            result.push_back(cur->val);
            cur = cur->right;
        }
        return result;
    }
};
