function partitionList(head, x) {
  const small = new ListNode(0);
  const large = new ListNode(0);
  let s = small;
  let l = large;
  for (let cur = head; cur !== null; cur = cur.next) {
    if (cur.val < x) {
      s.next = cur;
      s = cur;
    } else {
      l.next = cur;
      l = cur;
    }
  }
  l.next = null;
  s.next = large.next;
  return small.next;
}
