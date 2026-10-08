class Solution {
    ListNode* cur = nullptr;

    TreeNode* build(int lo, int hi) {
        if (lo > hi) return nullptr;
        int mid = (lo + hi + 1) / 2;
        TreeNode* left = build(lo, mid - 1);
        TreeNode* root = new TreeNode(cur->val);
        root->left = left;
        cur = cur->next;
        root->right = build(mid + 1, hi);
        return root;
    }

public:
    TreeNode* sortedListToBst(ListNode* head) {
        int n = 0;
        for (ListNode* node = head; node; node = node->next) n++;
        cur = head;
        return build(0, n - 1);
    }
};
