class Solution {
public:
    TreeNode* bstFromPreorder(vector<int>& preorder) {
        TreeNode* root = new TreeNode(preorder[0]);
        vector<TreeNode*> stack = {root};
        for (size_t i = 1; i < preorder.size(); i++) {
            int v = preorder[i];
            TreeNode* node = new TreeNode(v);
            if (v < stack.back()->val) {
                stack.back()->left = node;
            } else {
                TreeNode* parent = stack.back();
                stack.pop_back();
                while (!stack.empty() && stack.back()->val < v) {
                    parent = stack.back();
                    stack.pop_back();
                }
                parent->right = node;
            }
            stack.push_back(node);
        }
        return root;
    }
};
