class Solution {
public:
    vector<vector<int>> pathSum(TreeNode* root, int targetSum) {
        vector<vector<int>> result;
        if (!root) return result;
        vector<int> path;
        long long total = 0;
        // (node, leaving): leaving=true means "undo this node on the way back up"
        vector<pair<TreeNode*, bool>> stack{{root, false}};
        while (!stack.empty()) {
            auto [node, leaving] = stack.back();
            stack.pop_back();
            if (leaving) {
                path.pop_back();
                total -= node->val;
                continue;
            }
            path.push_back(node->val);
            total += node->val;
            stack.push_back({node, true});
            if (!node->left && !node->right) {
                if (total == targetSum) result.push_back(path);
            } else {
                // push right first so the left subtree is explored first
                if (node->right) stack.push_back({node->right, false});
                if (node->left) stack.push_back({node->left, false});
            }
        }
        return result;
    }
};
