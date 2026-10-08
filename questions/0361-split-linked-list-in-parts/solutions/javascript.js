function splitListToParts(head, k) {
  let n = 0;
  for (let p = head; p !== null; p = p.next) n++;
  const base = Math.floor(n / k);
  const extra = n % k;
  const parts = [];
  let cur = head;
  for (let i = 0; i < k; i++) {
    const size = base + (i < extra ? 1 : 0);
    const partHead = cur;
    for (let j = 0; j < size - 1; j++) cur = cur.next;
    if (size > 0) {
      const nxt = cur.next;
      cur.next = null;
      cur = nxt;
    }
    parts.push(partHead);
  }
  return parts;
}
