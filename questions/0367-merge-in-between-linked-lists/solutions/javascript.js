function mergeInBetween(list1, a, b, list2) {
  let before = list1;
  for (let i = 0; i < a - 1; i++) before = before.next;
  let after = before;
  for (let i = 0; i < b - a + 2; i++) after = after.next;
  before.next = list2;
  let tail = list2;
  while (tail.next !== null) tail = tail.next;
  tail.next = after;
  return list1;
}
