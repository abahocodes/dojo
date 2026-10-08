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

function assignTasks(servers, tasks) {
  // free: [weight, index]; busy: [freeTime, weight, index]
  const free = new MinHeap((x, y) => x[0] !== y[0] ? x[0] < y[0] : x[1] < y[1]);
  const busy = new MinHeap((x, y) =>
    x[0] !== y[0] ? x[0] < y[0] : x[1] !== y[1] ? x[1] < y[1] : x[2] < y[2]);
  servers.forEach((w, i) => free.push([w, i]));
  const result = [];
  let time = 0;
  for (let j = 0; j < tasks.length; j++) {
    time = Math.max(time, j);
    if (free.size === 0) time = Math.max(time, busy.peek()[0]);
    while (busy.size > 0 && busy.peek()[0] <= time) {
      const [, w, i] = busy.pop();
      free.push([w, i]);
    }
    const [w, i] = free.pop();
    result.push(i);
    busy.push([time + tasks[j], w, i]);
  }
  return result;
}
