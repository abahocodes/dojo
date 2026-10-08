class Solution {
public:
    vector<int> treeQueries(TreeNode* root, vector<int>& queries) {
        vector<pair<TreeNode*, int>> order;
        vector<pair<TreeNode*, int>> stack{{root, 0}};
        while (!stack.empty()) {
            auto [node, d] = stack.back();
            stack.pop_back();
            order.push_back({node, d});
            if (node->left) stack.push_back({node->left, d + 1});
            if (node->right) stack.push_back({node->right, d + 1});
        }
        int n = order.size();
        vector<int> depth(n + 1), height(n + 1);
        int levels = 0;
        for (int i = n - 1; i >= 0; i--) {
            auto [node, d] = order[i];
            depth[node->val] = d;
            levels = max(levels, d + 1);
            int h = 0;
            if (node->left) h = height[node->left->val] + 1;
            if (node->right) h = max(h, height[node->right->val] + 1);
            height[node->val] = h;
        }
        vector<int> best1(levels, -1), best2(levels, -1), owner(levels, 0);
        for (int v = 1; v <= n; v++) {
            int d = depth[v];
            int reach = d + height[v];
            if (reach > best1[d]) {
                best2[d] = best1[d];
                best1[d] = reach;
                owner[d] = v;
            } else if (reach > best2[d]) {
                best2[d] = reach;
            }
        }
        vector<int> answer;
        answer.reserve(queries.size());
        for (int q : queries) {
            int d = depth[q];
            if (owner[d] != q) answer.push_back(best1[d]);
            else if (best2[d] >= 0) answer.push_back(best2[d]);
            else answer.push_back(d - 1);
        }
        return answer;
    }
};
