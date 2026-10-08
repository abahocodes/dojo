def find_circle_num(is_connected: list[list[int]]) -> int:
    n = len(is_connected)
    parent = list(range(n))

    def find(x):
        while parent[x] != x:
            parent[x] = parent[parent[x]]
            x = parent[x]
        return x

    provinces = n
    for i in range(n):
        row = is_connected[i]
        for j in range(i + 1, n):
            if row[j] == 1:
                ri, rj = find(i), find(j)
                if ri != rj:
                    parent[ri] = rj
                    provinces -= 1
    return provinces
