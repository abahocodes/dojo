class Solution {
public:
    bool isEvenOddTree(TreeNode* root) {
        queue<TreeNode*> q;
        q.push(root);
        int depth = 0;
        while (!q.empty()) {
            bool evenLevel = depth % 2 == 0;
            // Sentinel just outside the value range [1, 10^6].
            int prev = evenLevel ? 0 : INT_MAX;
            int size = q.size();
            for (int i = 0; i < size; i++) {
                TreeNode* node = q.front();
                q.pop();
                int v = node->val;
                if (evenLevel) {
                    if (v % 2 == 0 || v <= prev) return false;
                } else {
                    if (v % 2 == 1 || v >= prev) return false;
                }
                prev = v;
                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
            depth++;
        }
        return true;
    }
};
