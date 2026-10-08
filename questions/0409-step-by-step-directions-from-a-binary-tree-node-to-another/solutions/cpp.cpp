class Solution {
public:
    string getDirections(TreeNode* root, int startValue, int destValue) {
        // parent[v] = (parent value, move from that parent down to v)
        unordered_map<int, pair<int, char>> parent;
        vector<TreeNode*> stack{root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            if (node->left != nullptr) {
                parent[node->left->val] = {node->val, 'L'};
                stack.push_back(node->left);
            }
            if (node->right != nullptr) {
                parent[node->right->val] = {node->val, 'R'};
                stack.push_back(node->right);
            }
        }
        auto pathFromRoot = [&](int value) {
            string moves;
            for (auto it = parent.find(value); it != parent.end(); it = parent.find(value)) {
                moves.push_back(it->second.second);
                value = it->second.first;
            }
            reverse(moves.begin(), moves.end());
            return moves;
        };
        string toStart = pathFromRoot(startValue);
        string toDest = pathFromRoot(destValue);
        size_t common = 0;
        while (common < toStart.size() && common < toDest.size() && toStart[common] == toDest[common]) {
            common++;
        }
        return string(toStart.size() - common, 'U') + toDest.substr(common);
    }
};
