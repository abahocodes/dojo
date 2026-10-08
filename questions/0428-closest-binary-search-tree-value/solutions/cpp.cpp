class Solution {
public:
    int closestValue(TreeNode* root, double target) {
        int best = root->val;
        TreeNode* node = root;
        while (node) {
            int v = node->val;
            double d = fabs(v - target);
            double bd = fabs(best - target);
            if (d < bd || (d == bd && v < best)) best = v;
            if (target < v) node = node->left;
            else if (target > v) node = node->right;
            else break;
        }
        return best;
    }
};
