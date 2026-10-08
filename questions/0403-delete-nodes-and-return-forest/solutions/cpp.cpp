class Solution {
public:
    vector<TreeNode*> delNodes(TreeNode* root, vector<int>& toDelete) {
        unordered_set<int> doomed(toDelete.begin(), toDelete.end());
        vector<TreeNode*> forest;
        // Each entry: the node, and whether it is the top of a tree once its parent is gone.
        vector<pair<TreeNode*, bool>> stack{{root, true}};
        while (!stack.empty()) {
            auto [node, isRoot] = stack.back();
            stack.pop_back();
            bool deleted = doomed.count(node->val) > 0;
            if (isRoot && !deleted) forest.push_back(node);
            if (node->left) {
                stack.push_back({node->left, deleted});
                if (doomed.count(node->left->val)) node->left = nullptr;
            }
            if (node->right) {
                stack.push_back({node->right, deleted});
                if (doomed.count(node->right->val)) node->right = nullptr;
            }
        }
        return forest;
    }
};
