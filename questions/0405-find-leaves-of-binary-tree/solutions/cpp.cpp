class Solution {
public:
    vector<vector<int>> findLeaves(TreeNode* root) {
        vector<vector<int>> result;
        height(root, result);
        return result;
    }

private:
    int height(TreeNode* node, vector<vector<int>>& result) {
        if (node == nullptr) return -1;
        int h = max(height(node->left, result), height(node->right, result)) + 1;
        if (h == (int)result.size()) result.emplace_back();
        result[h].push_back(node->val);
        return h;
    }
};
