function removeZeroSumSublists(head) {
  const dummy = new ListNode(0, head);
  const last = new Map();
  let total = 0;
  for (let node = dummy; node !== null; node = node.next) {
    total += node.val;
    last.set(total, node);
  }
  total = 0;
  for (let node = dummy; node !== null; node = node.next) {
    total += node.val;
    node.next = last.get(total).next;
  }
  return dummy.next;
}
