class Solution {
public:
    TreeNode* balanceBst(TreeNode* root) {
        vector<int> values;
        vector<TreeNode*> stack;
        TreeNode* node = root;
        while (!stack.empty() || node != nullptr) {
            while (node != nullptr) {
                stack.push_back(node);
                node = node->left;
            }
            node = stack.back();
            stack.pop_back();
            values.push_back(node->val);
            node = node->right;
        }
        return build(values, 0, (int)values.size() - 1);
    }

private:
    TreeNode* build(const vector<int>& values, int lo, int hi) {
        if (lo > hi) return nullptr;
        int mid = (lo + hi) / 2;
        return new TreeNode(values[mid], build(values, lo, mid - 1), build(values, mid + 1, hi));
    }
};
