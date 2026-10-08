class Solution {
public:
    ListNode* partitionList(ListNode* head, int x) {
        ListNode small(0);
        ListNode large(0);
        ListNode* s = &small;
        ListNode* l = &large;
        for (ListNode* cur = head; cur != nullptr; cur = cur->next) {
            if (cur->val < x) {
                s->next = cur;
                s = cur;
            } else {
                l->next = cur;
                l = cur;
            }
        }
        l->next = nullptr;
        s->next = large.next;
        return small.next;
    }
};
