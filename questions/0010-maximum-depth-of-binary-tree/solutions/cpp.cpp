class Solution {
public:
    int maxDepth(TreeNode* root) {
        if (!root) return 0;
        int depth = 0;
        vector<TreeNode*> level = {root};
        while (!level.empty()) {
            depth++;
            vector<TreeNode*> nxt;
            for (TreeNode* node : level) {
                if (node->left) nxt.push_back(node->left);
                if (node->right) nxt.push_back(node->right);
            }
            level = std::move(nxt);
        }
        return depth;
    }
};
