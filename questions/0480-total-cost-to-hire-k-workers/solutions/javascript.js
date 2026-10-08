class MinHeap {
  constructor(less) {
    this.a = [];
    this.less = less;
  }
  get size() {
    return this.a.length;
  }
  peek() {
    return this.a[0];
  }
  push(x) {
    const a = this.a;
    a.push(x);
    let i = a.length - 1;
    while (i > 0) {
      const p = (i - 1) >> 1;
      if (!this.less(a[i], a[p])) break;
      [a[i], a[p]] = [a[p], a[i]];
      i = p;
    }
  }
  pop() {
    const a = this.a;
    const top = a[0];
    const last = a.pop();
    if (a.length > 0) {
      a[0] = last;
      let i = 0;
      for (;;) {
        const l = 2 * i + 1;
        const r = l + 1;
        let m = i;
        if (l < a.length && this.less(a[l], a[m])) m = l;
        if (r < a.length && this.less(a[r], a[m])) m = r;
        if (m === i) break;
        [a[i], a[m]] = [a[m], a[i]];
        i = m;
      }
    }
    return top;
  }
}

function totalCost(costs, k, candidates) {
  // Entries are [cost, index]; equal costs go to the smaller index.
  const less = (x, y) => x[0] !== y[0] ? x[0] < y[0] : x[1] < y[1];
  const left = new MinHeap(less);
  const right = new MinHeap(less);
  let i = 0;
  let j = costs.length - 1;
  let total = 0;
  for (let round = 0; round < k; round++) {
    while (left.size < candidates && i <= j) left.push([costs[i], i++]);
    while (right.size < candidates && i <= j) right.push([costs[j], j--]);
    if (right.size === 0 || (left.size > 0 && less(left.peek(), right.peek()))) {
      total += left.pop()[0];
    } else {
      total += right.pop()[0];
    }
  }
  return total;
}
