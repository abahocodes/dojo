You are tracking scores as they come in and want to know the `k`-th largest
score at every moment.

You start with the scores in `nums`. Then the values in `adds` arrive one at a
time, in order. After inserting each new value, record the `k`-th largest
value among **all** scores so far (initial ones included). Duplicates count
separately: in `[8, 8, 5]` the first and second largest are both `8`.

Return the recorded values, one per element of `adds`, in arrival order. It is
guaranteed that at least `k` scores exist after every insertion.

## Example 1

```
k      = 3
nums   = [6, 2, 9, 4]
adds   = [5, 1, 10, 7, 7]
output = [5, 5, 6, 7, 7]
```

After adding `5` the scores are `[9, 6, 5, 4, 2]` (sorted), so the 3rd largest
is `5`. Adding `1` changes nothing near the top. After `10`, the top three are
`10, 9, 6`. After the first `7` they are `10, 9, 7`, and the second `7`
keeps the 3rd largest at `7`.

## Example 2

```
k      = 1
nums   = []
adds   = [-3, 4, -8]
output = [-3, 4, 4]
```

With `k = 1` you report the maximum so far; `-8` doesn't change it.

## Constraints

- `0 <= len(nums) <= 10^4`
- `0 <= len(adds) <= 10^4`
- `1 <= k <= 10^4`
- `-10^4 <= nums[i], adds[i] <= 10^4`
- After every insertion there are at least `k` scores.
