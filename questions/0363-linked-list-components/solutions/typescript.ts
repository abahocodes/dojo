function numComponents(head: ListNode | null, nums: number[]): number {
  const wanted = new Set<number>(nums);
  let count = 0;
  for (let cur = head; cur !== null; cur = cur.next) {
    if (wanted.has(cur.val) && (cur.next === null || !wanted.has(cur.next.val))) count++;
  }
  return count;
}
