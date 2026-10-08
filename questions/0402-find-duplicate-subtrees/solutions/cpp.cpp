class Solution {
public:
    vector<TreeNode*> findDuplicateSubtrees(TreeNode* root) {
        map<tuple<int, int, int>, int> ids;      // (left id, value, right id) -> subtree id
        unordered_map<int, int> count;           // subtree id -> occurrences
        unordered_map<TreeNode*, int> nodeId{{nullptr, 0}}; // node -> subtree id (0 means empty)
        vector<TreeNode*> result;
        vector<pair<TreeNode*, bool>> stack{{root, false}};
        while (!stack.empty()) {
            auto [node, done] = stack.back();
            stack.pop_back();
            if (!node) continue;
            if (!done) {
                stack.push_back({node, true});
                stack.push_back({node->right, false});
                stack.push_back({node->left, false});
                continue;
            }
            auto key = make_tuple(nodeId[node->left], node->val, nodeId[node->right]);
            auto it = ids.find(key);
            int sid;
            if (it == ids.end()) {
                sid = ids.size() + 1;
                ids[key] = sid;
            } else {
                sid = it->second;
            }
            nodeId[node] = sid;
            if (++count[sid] == 2) result.push_back(node);
        }
        return result;
    }
};
