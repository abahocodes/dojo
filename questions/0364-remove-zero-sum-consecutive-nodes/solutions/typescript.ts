function removeZeroSumSublists(head: ListNode | null): ListNode | null {
  const dummy = new ListNode(0, head);
  const last = new Map<number, ListNode>();
  let total = 0;
  for (let node: ListNode | null = dummy; node !== null; node = node.next) {
    total += node.val;
    last.set(total, node);
  }
  total = 0;
  for (let node: ListNode | null = dummy; node !== null; node = node.next) {
    total += node.val;
    node.next = last.get(total)!.next;
  }
  return dummy.next;
}
