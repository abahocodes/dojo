function accountsMerge(accounts) {
  const parent = accounts.map((_, i) => i);
  const find = (x) => {
    while (parent[x] !== x) {
      parent[x] = parent[parent[x]];
      x = parent[x];
    }
    return x;
  };

  const owner = new Map(); // email -> index of the first account that listed it
  accounts.forEach((account, i) => {
    for (let k = 1; k < account.length; k++) {
      const email = account[k];
      if (owner.has(email)) {
        const ri = find(i);
        const rj = find(owner.get(email));
        if (ri !== rj) parent[ri] = rj;
      } else {
        owner.set(email, i);
      }
    }
  });

  const groups = new Map();
  for (const [email, i] of owner) {
    const root = find(i);
    if (!groups.has(root)) groups.set(root, []);
    groups.get(root).push(email);
  }

  const cmp = (a, b) => (a < b ? -1 : a > b ? 1 : 0);
  const merged = [];
  for (const [root, emails] of groups) {
    emails.sort(cmp);
    merged.push([accounts[root][0], ...emails]);
  }
  merged.sort((a, b) => cmp(a[0], b[0]) || cmp(a[1], b[1]));
  return merged;
}
