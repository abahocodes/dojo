class Solution {
public:
    ListNode* reverseBetween(ListNode* head, int left, int right) {
        ListNode dummy(0, head);
        ListNode* before = &dummy;
        for (int i = 0; i < left - 1; i++) before = before->next;
        ListNode* tail = before->next;
        for (int i = 0; i < right - left; i++) {
            ListNode* move = tail->next;
            tail->next = move->next;
            move->next = before->next;
            before->next = move;
        }
        return dummy.next;
    }
};
