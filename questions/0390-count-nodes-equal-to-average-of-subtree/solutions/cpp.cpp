class Solution {
public:
    int averageOfSubtree(TreeNode* root) {
        vector<TreeNode*> order;
        vector<TreeNode*> stack{root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            order.push_back(node);
            if (node->left) stack.push_back(node->left);
            if (node->right) stack.push_back(node->right);
        }
        unordered_map<TreeNode*, pair<int, int>> totals;
        int count = 0;
        for (int i = (int)order.size() - 1; i >= 0; i--) {
            TreeNode* node = order[i];
            int s = node->val, c = 1;
            for (TreeNode* child : {node->left, node->right}) {
                if (child) {
                    s += totals[child].first;
                    c += totals[child].second;
                }
            }
            totals[node] = {s, c};
            if (s / c == node->val) count++;
        }
        return count;
    }
};
