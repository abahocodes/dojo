class Solution {
public:
    int sumEvenGrandparent(TreeNode* root) {
        int total = 0;
        vector<TreeNode*> stack{root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            for (TreeNode* child : {node->left, node->right}) {
                if (child == nullptr) continue;
                if (node->val % 2 == 0) {
                    if (child->left) total += child->left->val;
                    if (child->right) total += child->right->val;
                }
                stack.push_back(child);
            }
        }
        return total;
    }
};
