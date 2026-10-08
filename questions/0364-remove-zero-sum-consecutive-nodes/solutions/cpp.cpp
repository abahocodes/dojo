class Solution {
public:
    ListNode* removeZeroSumSublists(ListNode* head) {
        ListNode dummy(0, head);
        unordered_map<int, ListNode*> last;
        int total = 0;
        for (ListNode* node = &dummy; node != nullptr; node = node->next) {
            total += node->val;
            last[total] = node;
        }
        total = 0;
        for (ListNode* node = &dummy; node != nullptr; node = node->next) {
            total += node->val;
            node->next = last[total]->next;
        }
        return dummy.next;
    }
};
