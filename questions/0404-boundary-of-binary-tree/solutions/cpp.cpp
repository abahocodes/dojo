class Solution {
public:
    vector<int> boundaryOfBinaryTree(TreeNode* root) {
        vector<int> result{root->val};
        if (isLeaf(root)) return result;

        TreeNode* node = root->left;
        while (node != nullptr && !isLeaf(node)) {
            result.push_back(node->val);
            node = node->left != nullptr ? node->left : node->right;
        }

        vector<TreeNode*> stack{root};
        while (!stack.empty()) {
            TreeNode* cur = stack.back();
            stack.pop_back();
            if (isLeaf(cur)) {
                result.push_back(cur->val);
                continue;
            }
            if (cur->right != nullptr) stack.push_back(cur->right);
            if (cur->left != nullptr) stack.push_back(cur->left);
        }

        vector<int> right;
        node = root->right;
        while (node != nullptr && !isLeaf(node)) {
            right.push_back(node->val);
            node = node->right != nullptr ? node->right : node->left;
        }
        result.insert(result.end(), right.rbegin(), right.rend());
        return result;
    }

private:
    static bool isLeaf(TreeNode* node) {
        return node->left == nullptr && node->right == nullptr;
    }
};
