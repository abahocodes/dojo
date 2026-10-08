function deleteAllDuplicates(head) {
  const dummy = new ListNode(0, head);
  let prev = dummy;
  let cur = head;
  while (cur !== null) {
    if (cur.next !== null && cur.next.val === cur.val) {
      const v = cur.val;
      while (cur !== null && cur.val === v) cur = cur.next;
      prev.next = cur;
    } else {
      prev = cur;
      cur = cur.next;
    }
  }
  return dummy.next;
}
