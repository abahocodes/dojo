class MinHeap<T> {
  private a: T[] = [];
  private less: (x: T, y: T) => boolean;
  constructor(less: (x: T, y: T) => boolean) {
    this.less = less;
  }
  get size(): number {
    return this.a.length;
  }
  peek(): T {
    return this.a[0];
  }
  push(x: T): void {
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
  pop(): T {
    const a = this.a;
    const top = a[0];
    const last = a.pop() as T;
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

function totalCost(costs: number[], k: number, candidates: number): number {
  const less = (x: [number, number], y: [number, number]): boolean =>
    x[0] !== y[0] ? x[0] < y[0] : x[1] < y[1];
  const left = new MinHeap<[number, number]>(less);
  const right = new MinHeap<[number, number]>(less);
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
