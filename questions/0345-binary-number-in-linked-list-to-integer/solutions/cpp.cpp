class Solution {
public:
    int getDecimalValue(ListNode* head) {
        int value = 0;
        while (head) {
            value = (value << 1) | head->val;
            head = head->next;
        }
        return value;
    }
};
