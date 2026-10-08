class Solution {
public:
    bool findTarget(TreeNode* root, int k) {
        vector<int> values;
        vector<TreeNode*> stack;
        TreeNode* node = root;
        while (!stack.empty() || node) {
            while (node) {
                stack.push_back(node);
                node = node->left;
            }
            node = stack.back();
            stack.pop_back();
            values.push_back(node->val);
            node = node->right;
        }

        int i = 0, j = (int)values.size() - 1;
        while (i < j) {
            int s = values[i] + values[j];
            if (s == k) return true;
            if (s < k) i++;
            else j--;
        }
        return false;
    }
};
