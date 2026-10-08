function reverseKGroup(head, k) {
  const dummy = new ListNode(0, head);
  let groupPrev = dummy;
  while (true) {
    let kth = groupPrev;
    for (let i = 0; i < k; i++) {
      kth = kth.next;
      if (kth === null) return dummy.next;
    }
    const first = groupPrev.next;
    let prev = kth.next;
    let cur = first;
    for (let i = 0; i < k; i++) {
      const next = cur.next;
      cur.next = prev;
      prev = cur;
      cur = next;
    }
    groupPrev.next = kth;
    groupPrev = first;
  }
}
