class Solution {
public:
    TreeNode* createBinaryTree(vector<vector<int>>& descriptions) {
        unordered_map<int, TreeNode*> nodes;
        unordered_set<int> children;
        auto get = [&](int value) {
            auto it = nodes.find(value);
            if (it != nodes.end()) return it->second;
            TreeNode* node = new TreeNode(value);
            nodes[value] = node;
            return node;
        };
        for (auto& d : descriptions) {
            TreeNode* parent = get(d[0]);
            TreeNode* child = get(d[1]);
            if (d[2] == 1) parent->left = child;
            else parent->right = child;
            children.insert(d[1]);
        }
        for (auto& d : descriptions) {
            if (!children.count(d[0])) return nodes[d[0]];
        }
        return nullptr;
    }
};
