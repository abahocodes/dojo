class Solution {
public:
    bool hasCycle(ListNode* head) {
        // Floyd: the fast pointer laps the slow one only if there is a cycle.
        ListNode* slow = head;
        ListNode* fast = head;
        while (fast != nullptr && fast->next != nullptr) {
            slow = slow->next;
            fast = fast->next->next;
            if (slow == fast) return true;
        }
        return false;
    }

    // dojo calls this to run the tests; you don't need to change it.
    bool runHasCycle(vector<int>& values, int pos) {
        vector<ListNode*> nodes;
        for (int v : values) nodes.push_back(new ListNode(v));
        for (size_t i = 0; i + 1 < nodes.size(); i++) nodes[i]->next = nodes[i + 1];
        if (pos >= 0) nodes.back()->next = nodes[pos];
        bool result = hasCycle(nodes.empty() ? nullptr : nodes[0]);
        for (ListNode* node : nodes) delete node;
        return result;
    }
};
