function splitListToParts(head: ListNode | null, k: number): Array<ListNode | null> {
  let n = 0;
  for (let p = head; p !== null; p = p.next) n++;
  const base = Math.floor(n / k);
  const extra = n % k;
  const parts: Array<ListNode | null> = [];
  let cur: ListNode | null = head;
  for (let i = 0; i < k; i++) {
    const size = base + (i < extra ? 1 : 0);
    const partHead: ListNode | null = cur;
    for (let j = 0; j < size - 1; j++) cur = cur!.next;
    if (size > 0) {
      const nxt: ListNode | null = cur!.next;
      cur!.next = null;
      cur = nxt;
    }
    parts.push(partHead);
  }
  return parts;
}
