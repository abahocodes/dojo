class Solution {
public:
    ListNode* mergeKSortedLists(vector<ListNode*>& lists) {
        auto cmp = [](ListNode* a, ListNode* b) { return a->val > b->val; };
        priority_queue<ListNode*, vector<ListNode*>, decltype(cmp)> heap(cmp);
        for (ListNode* node : lists) {
            if (node) heap.push(node);
        }
        ListNode dummy(0);
        ListNode* curr = &dummy;
        while (!heap.empty()) {
            ListNode* node = heap.top();
            heap.pop();
            curr->next = node;
            curr = node;
            if (node->next) heap.push(node->next);
        }
        return dummy.next;
    }
};
