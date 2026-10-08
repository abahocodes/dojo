class Solution {
public:
    vector<int> closestKValues(TreeNode* root, double target, int k) {
        // pred: path to values <= target (top = largest); succ: values > target (top = smallest).
        vector<TreeNode*> pred, succ;
        TreeNode* node = root;
        while (node != nullptr) {
            if (node->val <= target) {
                pred.push_back(node);
                node = node->right;
            } else {
                succ.push_back(node);
                node = node->left;
            }
        }
        vector<int> result;
        result.reserve(k);
        for (int i = 0; i < k; i++) {
            if (succ.empty() || (!pred.empty() && target - pred.back()->val <= succ.back()->val - target)) {
                TreeNode* top = pred.back();
                pred.pop_back();
                for (TreeNode* cur = top->left; cur != nullptr; cur = cur->right) pred.push_back(cur);
                result.push_back(top->val);
            } else {
                TreeNode* top = succ.back();
                succ.pop_back();
                for (TreeNode* cur = top->right; cur != nullptr; cur = cur->left) succ.push_back(cur);
                result.push_back(top->val);
            }
        }
        return result;
    }
};
