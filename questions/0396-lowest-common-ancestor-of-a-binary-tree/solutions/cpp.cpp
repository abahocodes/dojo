class Solution {
public:
    int lowestCommonAncestor(TreeNode* root, int p, int q) {
        unordered_map<int, TreeNode*> parent;
        parent[root->val] = nullptr;
        vector<TreeNode*> stack{root};
        while (!stack.empty() && !(parent.count(p) && parent.count(q))) {
            TreeNode* node = stack.back();
            stack.pop_back();
            for (TreeNode* child : {node->left, node->right}) {
                if (child) {
                    parent[child->val] = node;
                    stack.push_back(child);
                }
            }
        }
        unordered_set<int> ancestors{p};
        for (TreeNode* up = parent[p]; up; up = parent[up->val]) ancestors.insert(up->val);
        int v = q;
        while (!ancestors.count(v)) v = parent[v]->val;
        return v;
    }
};
