class Solution {
public:
    TreeNode* buildTree(vector<int>& preorder, vector<int>& inorder) {
        if (preorder.empty()) return nullptr;
        TreeNode* root = new TreeNode(preorder[0]);
        vector<TreeNode*> stack{root};
        size_t j = 0; // next inorder position not yet closed off
        for (size_t i = 1; i < preorder.size(); i++) {
            int val = preorder[i];
            TreeNode* node = stack.back();
            if (node->val != inorder[j]) {
                // node's left subtree is not finished, so val is its left child
                node->left = new TreeNode(val);
                stack.push_back(node->left);
            } else {
                // pop every node whose left side is complete; the last one popped
                // is the node whose right child val is
                while (!stack.empty() && stack.back()->val == inorder[j]) {
                    node = stack.back();
                    stack.pop_back();
                    j++;
                }
                node->right = new TreeNode(val);
                stack.push_back(node->right);
            }
        }
        return root;
    }
};
