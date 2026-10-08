An array was first sorted in non-decreasing order (values **may repeat**) and
then **rotated**: some prefix of it, possibly empty, was cut off and moved to
the end. For example `[1, 2, 2, 4, 5, 5, 7]` rotated after its first four
elements becomes `[5, 5, 7, 1, 2, 2, 4]`. You receive only the rotated array
`nums`; the rotation point is unknown.

Return `true` if `target` occurs in `nums`, and `false` otherwise.

Aim for `O(log n)` on typical inputs. Because of duplicates, some inputs
(such as an array that is almost entirely one value) force `O(n)` in the
worst case, and that is acceptable.

## Example 1

```
nums   = [5, 5, 7, 1, 2, 2, 4]
target = 2
output = true
```

## Example 2

```
nums   = [3, 3, 1, 3, 3, 3]
target = 2
output = false
```

## Constraints

- `1 <= len(nums) <= 5000`
- `-10^4 <= nums[i], target <= 10^4`
- `nums` is a rotation of an array sorted in non-decreasing order
