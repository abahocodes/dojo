function swapPairs(head: ListNode | null): ListNode | null {
  const dummy = new ListNode(0, head);
  let prev: ListNode = dummy;
  while (prev.next !== null && prev.next.next !== null) {
    const a: ListNode = prev.next;
    const b: ListNode = prev.next.next;
    a.next = b.next;
    b.next = a;
    prev.next = b;
    prev = a;
  }
  return dummy.next;
}
