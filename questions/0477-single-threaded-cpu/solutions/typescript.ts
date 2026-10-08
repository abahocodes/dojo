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

function getOrder(tasks: number[][]): number[] {
  const n = tasks.length;
  const byEnqueue = Array.from({ length: n }, (_, i) => i);
  byEnqueue.sort((a, b) => tasks[a][0] - tasks[b][0] || a - b);
  const ready = new MinHeap<[number, number]>((x, y) => x[0] !== y[0] ? x[0] < y[0] : x[1] < y[1]);
  const order: number[] = [];
  let time = 0;
  let p = 0;
  while (order.length < n) {
    if (ready.size === 0 && time < tasks[byEnqueue[p]][0]) time = tasks[byEnqueue[p]][0];
    while (p < n && tasks[byEnqueue[p]][0] <= time) {
      const i = byEnqueue[p++];
      ready.push([tasks[i][1], i]);
    }
    const [duration, i] = ready.pop();
    time += duration;
    order.push(i);
  }
  return order;
}
