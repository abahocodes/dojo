def accounts_merge(accounts: list[list[str]]) -> list[list[str]]:
    parent = list(range(len(accounts)))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    owner = {}  # email -> index of the first account that listed it
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

    merged = [[accounts[root][0]] + sorted(emails) for root, emails in groups.items()]
    merged.sort(key=lambda account: (account[0], account[1]))
    return merged
