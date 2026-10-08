class Solution {
public:
    ListNode* insertionSortList(ListNode* head) {
        ListNode dummy(0);
        ListNode* tail = nullptr;
        ListNode* cur = head;
        while (cur) {
            ListNode* nxt = cur->next;
            if (tail && tail->val <= cur->val) {
                tail->next = cur;
                cur->next = nullptr;
                tail = cur;
            } else {
                ListNode* p = &dummy;
                while (p->next && p->next->val <= cur->val) p = p->next;
                cur->next = p->next;
                p->next = cur;
                if (!cur->next) tail = cur;
            }
            cur = nxt;
        }
        return dummy.next;
    }
};
