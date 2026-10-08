class Solution {
public:
    vector<double> averageOfLevels(TreeNode* root) {
        vector<double> averages;
        queue<TreeNode*> q;
        q.push(root);
        while (!q.empty()) {
            int size = q.size();
            long long total = 0;
            for (int i = 0; i < size; i++) {
                TreeNode* node = q.front();
                q.pop();
                total += node->val;
                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
            averages.push_back((double) total / size);
        }
        return averages;
    }
};
