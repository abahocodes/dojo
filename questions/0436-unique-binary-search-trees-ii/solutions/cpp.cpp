class Solution {
public:
    vector<TreeNode*> generateTrees(int n) {
        memo.clear();
        return build(1, n);
    }

private:
    map<pair<int, int>, vector<TreeNode*>> memo;

    vector<TreeNode*> build(int lo, int hi) {
        if (lo > hi) return {nullptr};
        auto it = memo.find({lo, hi});
        if (it != memo.end()) return it->second;
        vector<TreeNode*> trees;
        for (int v = lo; v <= hi; v++) {
            vector<TreeNode*> lefts = build(lo, v - 1);
            vector<TreeNode*> rights = build(v + 1, hi);
            for (TreeNode* left : lefts) {
                for (TreeNode* right : rights) {
                    trees.push_back(new TreeNode(v, left, right));
                }
            }
        }
        memo[{lo, hi}] = trees;
        return trees;
    }
};
