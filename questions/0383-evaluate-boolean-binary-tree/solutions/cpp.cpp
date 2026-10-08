class Solution {
public:
    bool evaluateTree(TreeNode* root) {
        unordered_map<TreeNode*, bool> value;
        vector<pair<TreeNode*, bool>> stack = {{root, false}};
        while (!stack.empty()) {
            auto [node, childrenDone] = stack.back();
            stack.pop_back();
            if (!node->left) {
                value[node] = node->val == 1;
            } else if (childrenDone) {
                bool left = value[node->left];
                bool right = value[node->right];
                value[node] = node->val == 2 ? (left || right) : (left && right);
            } else {
                stack.push_back({node, true});
                stack.push_back({node->left, false});
                stack.push_back({node->right, false});
            }
        }
        return value[root];
    }
};
