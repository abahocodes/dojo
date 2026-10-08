class Solution {
public:
    ListNode* deleteNodes(ListNode* head, int m, int n) {
        ListNode* cur = head;
        while (cur != nullptr) {
            for (int i = 0; i < m - 1; i++) {
                if (cur->next == nullptr) return head;
                cur = cur->next;
            }
            ListNode* skip = cur->next;
            for (int i = 0; i < n && skip != nullptr; i++) {
                skip = skip->next;
            }
            cur->next = skip;
            cur = skip;
        }
        return head;
    }
};
