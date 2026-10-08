function insertionSortList(head) {
  const dummy = new ListNode(0);
  let tail = null;
  let cur = head;
  while (cur !== null) {
    const nxt = cur.next;
    if (tail !== null && tail.val <= cur.val) {
      tail.next = cur;
      cur.next = null;
      tail = cur;
    } else {
      let p = dummy;
      while (p.next !== null && p.next.val <= cur.val) p = p.next;
      cur.next = p.next;
      p.next = cur;
      if (cur.next === null) tail = cur;
    }
    cur = nxt;
  }
  return dummy.next;
}
