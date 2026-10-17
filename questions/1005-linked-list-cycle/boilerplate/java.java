// 1005. Linked List Cycle (easy)
// /show to reread the problem · /test runs visible tests · /submit when done

// Provided by dojo:
// class ListNode {
//     int val;
//     ListNode next;
//     ListNode() {}
//     ListNode(int val) { this.val = val; }
//     ListNode(int val, ListNode next) { this.val = val; this.next = next; }
// }

class Solution {
    public boolean hasCycle(ListNode head) {
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
