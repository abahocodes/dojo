from collections import deque


def longest_subarray_limit(nums, limit):
    maxq, minq = deque(), deque()
    left = 0
    best = 0
    for right, x in enumerate(nums):
        while maxq and nums[maxq[-1]] < x:
            maxq.pop()
        maxq.append(right)
        while minq and nums[minq[-1]] > x:
            minq.pop()
        minq.append(right)
        while nums[maxq[0]] - nums[minq[0]] > limit:
            left += 1
            if maxq[0] < left:
                maxq.popleft()
            if minq[0] < left:
                minq.popleft()
        best = max(best, right - left + 1)
    return best
