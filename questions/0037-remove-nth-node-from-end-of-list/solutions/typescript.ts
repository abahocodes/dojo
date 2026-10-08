function removeNthFromEnd(head: ListNode | null, n: number): ListNode | null {
    const dummy = new ListNode(0, head);
    let fast: ListNode = dummy;
    let slow: ListNode = dummy;
    for (let k = 0; k < n; k++) {
        fast = fast.next!;
    }
    while (fast.next !== null) {
        fast = fast.next;
        slow = slow.next!;
    }
    slow.next = slow.next!.next;
    return dummy.next;
}
