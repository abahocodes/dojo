function mergeNodes(head: ListNode | null): ListNode | null {
  const dummy = new ListNode(0);
  let tail: ListNode = dummy;
  let total = 0;
  let cur: ListNode | null = head!.next;
  while (cur !== null) {
    if (cur.val === 0) {
      cur.val = total;
      tail.next = cur;
      tail = cur;
      total = 0;
    } else {
      total += cur.val;
    }
    cur = cur.next;
  }
  tail.next = null;
  return dummy.next;
}
