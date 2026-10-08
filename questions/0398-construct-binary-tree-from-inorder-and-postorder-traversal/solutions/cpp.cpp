class Solution {
public:
    TreeNode* buildTreeInPost(vector<int>& inorder, vector<int>& postorder) {
        int n = postorder.size();
        if (n == 0) return nullptr;
        // Walk postorder backwards (root, right, left) and inorder backwards.
        TreeNode* root = new TreeNode(postorder[n - 1]);
        vector<TreeNode*> stack{root};
        int i = n - 1;
        for (int j = n - 2; j >= 0; j--) {
            TreeNode* node = new TreeNode(postorder[j]);
            TreeNode* parent = stack.back();
            if (parent->val != inorder[i]) {
                parent->right = node;
            } else {
                while (!stack.empty() && stack.back()->val == inorder[i]) {
                    parent = stack.back();
                    stack.pop_back();
                    i--;
                }
                parent->left = node;
            }
            stack.push_back(node);
        }
        return root;
    }
};
