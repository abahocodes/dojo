class Solution {
public:
    vector<int> findMode(TreeNode* root) {
        vector<int> modes;
        int best = 0, count = 0, prev = 0;
        bool hasPrev = false;
        vector<TreeNode*> stack;
        TreeNode* node = root;
        while (!stack.empty() || node) {
            while (node) {
                stack.push_back(node);
                node = node->left;
            }
            node = stack.back();
            stack.pop_back();
            count = (hasPrev && node->val == prev) ? count + 1 : 1;
            prev = node->val;
            hasPrev = true;
            if (count > best) {
                best = count;
                modes.assign(1, node->val);
            } else if (count == best) {
                modes.push_back(node->val);
            }
            node = node->right;
        }
        return modes;
    }
};
