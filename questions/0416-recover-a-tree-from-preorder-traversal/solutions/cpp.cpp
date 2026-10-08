class Solution {
public:
    TreeNode* recoverFromPreorder(string& traversal) {
        vector<TreeNode*> stack;
        size_t n = traversal.size(), i = 0;
        while (i < n) {
            size_t depth = 0;
            while (traversal[i] == '-') {
                depth++;
                i++;
            }
            int value = 0;
            while (i < n && traversal[i] != '-') {
                value = value * 10 + (traversal[i] - '0');
                i++;
            }
            TreeNode* node = new TreeNode(value);
            stack.resize(depth);
            if (!stack.empty()) {
                TreeNode* parent = stack.back();
                if (parent->left == nullptr) parent->left = node;
                else parent->right = node;
            }
            stack.push_back(node);
        }
        return stack[0];
    }
};
