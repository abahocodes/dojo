class Solution {
public:
    vector<int> distanceK(TreeNode* root, int target, int k) {
        // Turn the tree into an undirected graph keyed by value.
        unordered_map<int, vector<int>> adj;
        adj[root->val];
        vector<TreeNode*> stack{root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            for (TreeNode* child : {node->left, node->right}) {
                if (child) {
                    adj[node->val].push_back(child->val);
                    adj[child->val].push_back(node->val);
                    stack.push_back(child);
                }
            }
        }
        vector<int> frontier{target};
        unordered_set<int> seen{target};
        for (int step = 0; step < k && !frontier.empty(); step++) {
            vector<int> next;
            for (int v : frontier) {
                for (int w : adj[v]) {
                    if (seen.insert(w).second) next.push_back(w);
                }
            }
            frontier = move(next);
        }
        return frontier;
    }
};
