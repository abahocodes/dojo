# Approach: union-find on accounts, keyed by email

Think of each account as a node, with an edge between two accounts whenever
they share an email. A person is a connected component, including the
transitive chains.

You never need to compare accounts pairwise. Keep a map `owner` from each
email to the first account that listed it. When another account lists the
same email, union the two accounts. After one pass, every account of a person
shares a union-find root.

Then collect every distinct email under the root of its owner, sort each
group, and prepend the name. Finally sort the merged accounts by
`(name, first email)`. Every email belongs to exactly one person, so no two
merged accounts share a first email and this ordering is unambiguous.

```python
def accounts_merge(accounts):
    parent = list(range(len(accounts)))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    owner = {}
    for i, account in enumerate(accounts):
        for email in account[1:]:
            if email in owner:
                ri, rj = find(i), find(owner[email])
                if ri != rj:
                    parent[ri] = rj
            else:
                owner[email] = i

    groups = {}
    for email, i in owner.items():
        groups.setdefault(find(i), []).append(email)

    merged = [[accounts[root][0]] + sorted(emails)
              for root, emails in groups.items()]
    merged.sort(key=lambda acc: (acc[0], acc[1]))
    return merged
```

**Alternative:** build an email graph (link every email of an account to the
account's first email) and run DFS/BFS from each unvisited email to collect a
component.

## Complexity

- Time: O(E log E), where E is the total number of emails: sorting dominates;
  the union-find work is nearly linear.
- Space: O(E) for the map and the groups.

## Pitfalls

- Merging by **name** is wrong: two different people can share a name
  (Example 1 has two separate Nias).
- Emails can repeat, even within one account. Keep only distinct addresses.
- Merging only accounts that directly share an email misses chains. Union-find
  (or a graph search) handles transitivity.
- The output order is part of the answer: emails sorted inside each account,
  accounts sorted by name and then first email. In JavaScript, sort with an
  explicit `<`/`>` comparator; `localeCompare` may order uppercase and
  punctuation differently from plain character codes.
