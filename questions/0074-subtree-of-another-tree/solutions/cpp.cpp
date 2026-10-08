class Solution {
public:
    bool isSubtree(TreeNode* root, TreeNode* subRoot) {
        return encode(root).find(encode(subRoot)) != string::npos;
    }

private:
    // pre-order with explicit null markers; the leading comma on every token
    // keeps "2" from matching inside "12"
    static string encode(TreeNode* root) {
        string out;
        vector<TreeNode*> stack{root};
        while (!stack.empty()) {
            TreeNode* node = stack.back();
            stack.pop_back();
            if (!node) {
                out += ",#";
            } else {
                out += ',';
                out += to_string(node->val);
                stack.push_back(node->right);
                stack.push_back(node->left);
            }
        }
        return out;
    }
};
