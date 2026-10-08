def exclusive_time(n, logs):
    result = [0] * n
    stack = []
    prev = 0
    for entry in logs:
        fid, kind, ts = entry.split(":")
        fid, t = int(fid), int(ts)
        if kind == "start":
            if stack:
                result[stack[-1]] += t - prev
            stack.append(fid)
            prev = t
        else:
            result[stack.pop()] += t - prev + 1
            prev = t + 1
    return result
