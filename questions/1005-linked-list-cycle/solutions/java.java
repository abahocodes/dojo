class Solution {
    public boolean hasCycle(ListNode head) {
        // Floyd: the fast pointer laps the slow one only if there is a cycle.
        ListNode slow = head, fast = head;
        while (fast != null && fast.next != null) {
            slow = slow.next;
            fast = fast.next.next;
            if (slow == fast) return true;
        }
        return false;
    }

    // dojo calls this to run the tests; you don't need to change it.
    public boolean runHasCycle(int[] values, int pos) {
        ListNode[] nodes = new ListNode[values.length];
        for (int i = 0; i < values.length; i++) nodes[i] = new ListNode(values[i]);
        for (int i = 0; i + 1 < nodes.length; i++) nodes[i].next = nodes[i + 1];
        if (pos >= 0) nodes[nodes.length - 1].next = nodes[pos];
        return hasCycle(nodes.length > 0 ? nodes[0] : null);
    }
}
