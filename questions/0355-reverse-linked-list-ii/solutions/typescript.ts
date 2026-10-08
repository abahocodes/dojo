function reverseBetween(head: ListNode | null, left: number, right: number): ListNode | null {
  const dummy = new ListNode(0, head);
  let before: ListNode = dummy;
  for (let i = 0; i < left - 1; i++) before = before.next!;
  const tail: ListNode = before.next!;
  for (let i = 0; i < right - left; i++) {
    const move: ListNode = tail.next!;
    tail.next = move.next;
    move.next = before.next;
    before.next = move;
  }
  return dummy.next;
}
