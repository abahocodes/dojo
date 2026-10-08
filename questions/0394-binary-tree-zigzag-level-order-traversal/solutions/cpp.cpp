class Solution {
public:
    vector<vector<int>> zigzagLevelOrder(TreeNode* root) {
        vector<vector<int>> result;
        queue<TreeNode*> q;
        if (root) q.push(root);
        bool leftToRight = true;
        while (!q.empty()) {
            int size = q.size();
            vector<int> values(size);
            for (int i = 0; i < size; i++) {
                TreeNode* node = q.front();
                q.pop();
                values[leftToRight ? i : size - 1 - i] = node->val;
                if (node->left) q.push(node->left);
                if (node->right) q.push(node->right);
            }
            result.push_back(move(values));
            leftToRight = !leftToRight;
        }
        return result;
    }
};
