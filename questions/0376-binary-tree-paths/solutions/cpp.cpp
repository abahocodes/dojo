class Solution {
public:
    vector<string> binaryTreePaths(TreeNode* root) {
        vector<string> paths;
        vector<pair<TreeNode*, string>> stack;
        stack.push_back({root, to_string(root->val)});
        while (!stack.empty()) {
            auto [node, path] = stack.back();
            stack.pop_back();
            if (node->left == nullptr && node->right == nullptr) {
                paths.push_back(path);
                continue;
            }
            if (node->right) stack.push_back({node->right, path + "->" + to_string(node->right->val)});
            if (node->left) stack.push_back({node->left, path + "->" + to_string(node->left->val)});
        }
        return paths;
    }
};
