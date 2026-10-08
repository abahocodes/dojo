class Solution {
public:
    vector<int> getAllElements(TreeNode* root1, TreeNode* root2) {
        vector<int> a = inorder(root1);
        vector<int> b = inorder(root2);
        vector<int> merged;
        merged.reserve(a.size() + b.size());
        size_t i = 0, j = 0;
        while (i < a.size() && j < b.size()) {
            if (a[i] <= b[j]) merged.push_back(a[i++]);
            else merged.push_back(b[j++]);
        }
        while (i < a.size()) merged.push_back(a[i++]);
        while (j < b.size()) merged.push_back(b[j++]);
        return merged;
    }

private:
    vector<int> inorder(TreeNode* root) {
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
        return values;
    }
};
