class Solution {
public:
    string tree2str(TreeNode* root) {
        // each entry is either a node to print or (nullptr, literal text)
        string out;
        vector<pair<TreeNode*, const char*>> stack = {{root, nullptr}};
        while (!stack.empty()) {
            auto [node, text] = stack.back();
            stack.pop_back();
            if (!node) {
                out += text;
                continue;
            }
            out += to_string(node->val);
            if (node->right) {
                stack.push_back({nullptr, ")"});
                stack.push_back({node->right, nullptr});
                stack.push_back({nullptr, "("});
            }
            if (node->left) {
                stack.push_back({nullptr, ")"});
                stack.push_back({node->left, nullptr});
                stack.push_back({nullptr, "("});
            } else if (node->right) {
                stack.push_back({nullptr, "()"});
            }
        }
        return out;
    }
};
