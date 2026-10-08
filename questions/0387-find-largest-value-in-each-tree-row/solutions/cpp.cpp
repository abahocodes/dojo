class Solution {
public:
    vector<int> largestValues(TreeNode* root) {
        vector<int> result;
        queue<TreeNode*> q;
        if (root) q.push(root);
        while (!q.empty()) {
            int size = q.size();
            int best = INT_MIN;
            for (int i = 0; i < size; i++) {
                TreeNode* node = q.front();
                q.pop();
                best = max(best, node->val);
                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
            result.push_back(best);
        }
        return result;
    }
};
