class Solution {
public:
    vector<vector<int>> closestNodes(TreeNode* root, vector<int>& queries) {
        vector<int> values;
        vector<TreeNode*> stack;
        TreeNode* node = root;
        while (!stack.empty() || node != nullptr) {
            while (node != nullptr) {
                stack.push_back(node);
                node = node->left;
            }
            node = stack.back();
            stack.pop_back();
            values.push_back(node->val);
            node = node->right;
        }
        vector<vector<int>> answer;
        answer.reserve(queries.size());
        for (int q : queries) {
            auto it = lower_bound(values.begin(), values.end(), q);
            if (it != values.end() && *it == q) {
                answer.push_back({q, q});
            } else {
                int floorVal = it != values.begin() ? *prev(it) : -1;
                int ceilVal = it != values.end() ? *it : -1;
                answer.push_back({floorVal, ceilVal});
            }
        }
        return answer;
    }
};
