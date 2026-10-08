class Solution {
public:
    int amountOfTime(TreeNode* root, int start) {
        unordered_map<TreeNode*, TreeNode*> parent{{root, nullptr}};
        TreeNode* source = nullptr;
        vector<TreeNode*> stack{root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            if (node->val == start) source = node;
            for (TreeNode* child : {node->left, node->right}) {
                if (child != nullptr) {
                    parent[child] = node;
                    stack.push_back(child);
                }
            }
        }

        unordered_set<TreeNode*> seen{source};
        vector<TreeNode*> frontier{source};
        int minutes = -1;
        while (!frontier.empty()) {
            minutes++;
            vector<TreeNode*> next;
            for (TreeNode* node : frontier) {
                for (TreeNode* neighbor : {node->left, node->right, parent[node]}) {
                    if (neighbor != nullptr && seen.insert(neighbor).second) next.push_back(neighbor);
                }
            }
            frontier = move(next);
        }
        return minutes;
    }
};
