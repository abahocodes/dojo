function reverseBetween(head, left, right) {
  const dummy = new ListNode(0, head);
  let before = dummy;
  for (let i = 0; i < left - 1; i++) before = before.next;
  const tail = before.next;
  for (let i = 0; i < right - left; i++) {
    const move = tail.next;
    tail.next = move.next;
    move.next = before.next;
    before.next = move;
  }
  return dummy.next;
}
