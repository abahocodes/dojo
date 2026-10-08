class Solution {
public:
    ListNode* mergeInBetween(ListNode* list1, int a, int b, ListNode* list2) {
        ListNode* before = list1;
        for (int i = 0; i < a - 1; i++) before = before->next;
        ListNode* after = before;
        for (int i = 0; i < b - a + 2; i++) after = after->next;
        before->next = list2;
        ListNode* tail = list2;
        while (tail->next != nullptr) tail = tail->next;
        tail->next = after;
        return list1;
    }
};
