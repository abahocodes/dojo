class Solution {
public:
    TreeNode* constructFromPrePost(vector<int>& preorder, vector<int>& postorder) {
        TreeNode* root = new TreeNode(preorder[0]);
        vector<TreeNode*> stack{root};
        size_t j = 0;
        for (size_t i = 1; i < preorder.size(); i++) {
            TreeNode* node = new TreeNode(preorder[i]);
            // Pop every node whose subtree is already complete.
            while (stack.back()->val == postorder[j]) {
                stack.pop_back();
                j++;
            }
            TreeNode* parent = stack.back();
            if (!parent->left) parent->left = node;
            else parent->right = node;
            stack.push_back(node);
        }
        return root;
    }
};
