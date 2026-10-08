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

function findMaximizedCapital(k, w, profits, capital) {
  const order = Array.from({ length: profits.length }, (_, i) => i);
  order.sort((a, b) => capital[a] - capital[b]);
  const affordable = new MinHeap((x, y) => x > y); // max-heap of profits
  let p = 0;
  for (let round = 0; round < k; round++) {
    while (p < order.length && capital[order[p]] <= w) affordable.push(profits[order[p++]]);
    if (affordable.size === 0) break;
    w += affordable.pop();
  }
  return w;
}
