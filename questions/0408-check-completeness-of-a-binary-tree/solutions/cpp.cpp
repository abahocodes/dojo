class Solution {
public:
    bool isCompleteTree(TreeNode* root) {
        queue<TreeNode*> q;
        q.push(root);
        bool seenGap = false;
        while (!q.empty()) {
            TreeNode* node = q.front();
            q.pop();
            if (node == nullptr) {
                seenGap = true;
                continue;
            }
            if (seenGap) return false;
            q.push(node->left);
            q.push(node->right);
        }
        return true;
    }
};
