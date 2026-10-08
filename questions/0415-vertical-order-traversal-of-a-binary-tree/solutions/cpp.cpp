class Solution {
public:
    vector<vector<int>> verticalTraversal(TreeNode* root) {
        vector<array<int, 3>> entries; // {col, row, val}
        vector<tuple<TreeNode*, int, int>> stack{{root, 0, 0}};
        while (!stack.empty()) {
            auto [node, row, col] = stack.back();
            stack.pop_back();
            entries.push_back({col, row, node->val});
            if (node->left) stack.push_back({node->left, row + 1, col - 1});
            if (node->right) stack.push_back({node->right, row + 1, col + 1});
        }
        sort(entries.begin(), entries.end());
        vector<vector<int>> result;
        for (size_t i = 0; i < entries.size(); i++) {
            if (i == 0 || entries[i][0] != entries[i - 1][0]) result.emplace_back();
            result.back().push_back(entries[i][2]);
        }
        return result;
    }
};
