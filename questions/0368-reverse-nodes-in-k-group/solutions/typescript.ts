function reverseKGroup(head: ListNode | null, k: number): ListNode | null {
  const dummy = new ListNode(0, head);
  let groupPrev: ListNode = dummy;
  while (true) {
    let kth: ListNode | null = groupPrev;
    for (let i = 0; i < k; i++) {
      kth = kth!.next;
      if (kth === null) return dummy.next;
    }
    const first: ListNode = groupPrev.next!;
    let prev: ListNode | null = kth!.next;
    let cur: ListNode = first;
    for (let i = 0; i < k; i++) {
      const next: ListNode | null = cur.next;
      cur.next = prev;
      prev = cur;
      cur = next!;
    }
    groupPrev.next = kth;
    groupPrev = first;
  }
}
