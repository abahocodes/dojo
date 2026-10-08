class Solution {
public:
    TreeNode* canMerge(vector<TreeNode*>& trees) {
        unordered_map<int, TreeNode*> roots;
        unordered_set<int> leafValues;
        for (TreeNode* t : trees) {
            roots[t->val] = t;
            if (t->left) leafValues.insert(t->left->val);
            if (t->right) leafValues.insert(t->right->val);
        }

        TreeNode* root = nullptr;
        int candidates = 0;
        for (TreeNode* t : trees) {
            if (!leafValues.count(t->val)) {
                root = t;
                candidates++;
            }
        }
        if (candidates != 1) return nullptr;
        roots.erase(root->val);

        // Iterative DFS carrying the open interval (lo, hi) each node must fit in.
        vector<tuple<TreeNode*, long long, long long>> stack;
        stack.emplace_back(root, 0, 1LL << 31);
        while (!stack.empty()) {
            auto [node, lo, hi] = stack.back();
            stack.pop_back();
            if (node->val <= lo || node->val >= hi) return nullptr;
            if (!node->left && !node->right) {
                auto it = roots.find(node->val);
                if (it != roots.end()) {
                    node->left = it->second->left;
                    node->right = it->second->right;
                    roots.erase(it);
                }
            }
            if (node->left) stack.emplace_back(node->left, lo, node->val);
            if (node->right) stack.emplace_back(node->right, node->val, hi);
        }

        return roots.empty() ? root : nullptr;
    }
};
