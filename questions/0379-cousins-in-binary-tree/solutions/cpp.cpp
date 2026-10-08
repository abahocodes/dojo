class Solution {
public:
    bool isCousins(TreeNode* root, int x, int y) {
        vector<TreeNode*> level = {root};
        while (!level.empty()) {
            TreeNode* parentX = nullptr;
            TreeNode* parentY = nullptr;
            vector<TreeNode*> next;
            for (TreeNode* node : level) {
                for (TreeNode* child : {node->left, node->right}) {
                    if (child == nullptr) continue;
                    if (child->val == x) parentX = node;
                    else if (child->val == y) parentY = node;
                    next.push_back(child);
                }
            }
            if (parentX && parentY) return parentX != parentY;
            if (parentX || parentY) return false;
            level = move(next);
        }
        return false;
    }
};
