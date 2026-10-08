class Solution {
public:
    int pathSumCount(TreeNode* root, int targetSum) {
        if (!root) return 0;
        unordered_map<long long, int> seen{{0, 1}}; // prefix sums on the current root-to-node path
        int count = 0;
        struct Frame {
            TreeNode* node;
            long long before;
            bool leaving;
        };
        vector<Frame> stack{{root, 0, false}};
        while (!stack.empty()) {
            Frame f = stack.back();
            stack.pop_back();
            long long prefix = f.before + f.node->val;
            if (f.leaving) {
                seen[prefix]--;
                continue;
            }
            auto it = seen.find(prefix - targetSum);
            if (it != seen.end()) count += it->second;
            seen[prefix]++;
            stack.push_back({f.node, f.before, true});
            if (f.node->right) stack.push_back({f.node->right, prefix, false});
            if (f.node->left) stack.push_back({f.node->left, prefix, false});
        }
        return count;
    }
};
