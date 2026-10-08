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
        // Left before right: argument evaluation order is unspecified in C++.
        int left = height(node->left, result);
        int right = height(node->right, result);
        int h = max(left, right) + 1;
        if (h == (int)result.size()) result.emplace_back();
        result[h].push_back(node->val);
        return h;
    }
};
