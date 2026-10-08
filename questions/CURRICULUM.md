# Curriculum

This is the plan for growing dojo's question bank from 115 to 1000 problems.
It is meant to be read by contributors: it shows what exists, what is planned,
and why the bank is shaped the way it is.

## Intent

Someone who works through all 1000 questions should be solidly prepared for any
coding interview: FAANG and other large companies, startups, and quant/fintech
shops, from new grad through senior, including the occasional harder
competitive-programming-style round.

That goal drives three choices:

- **Breadth.** Every topic that interviews actually test is covered, from arrays
  and hashing through graphs, dynamic programming, strings, math, and range-query
  data structures, plus a set of advanced techniques (LIS in O(n log n),
  monotonic-deque DP, meet-in-the-middle, sweep line, Eulerian paths, strongly
  connected components, bridges and articulation points).
- **Depth where interviews are heavy.** Arrays, two pointers, sliding window,
  trees, graphs, and DP get the most problems, with several problems per
  sub-pattern, so a pattern is practised until it is recognised and not merely
  seen once.
- **Well-known problems first.** The bank favours the problems interviewers
  actually ask, under their common names, then classic variants that each teach
  a distinct technique. Near-duplicates (same core solution, cosmetic changes)
  are left out.

## Difficulty mix

Across the whole bank the target is about 25% easy, 55% medium and 20% hard.
Easy problems teach a pattern in isolation, mediums are the bulk of real
interviews, and hards cover the stretch rounds and the competitive-style
questions some companies ask.

## Order and batches

Planned IDs (116–1000) are grouped by topic, and within a topic run from easy to
hard, with closely related problems next to each other. Any run of eight
consecutive IDs is therefore a coherent batch to write together.

## What every question must fit

Every question is a single pure function over dojo's types: `int` (32-bit),
`long` (64-bit, up to 2^53), `float`, `bool`, `string`, `ListNode`, `TreeNode`,
and lists of these. Maps, sets and objects are modelled as arrays, graphs as
edge lists, adjacency arrays or grids. Each answer is unique, or made unique by
an explicit rule (sorted output, lexicographically smallest, or an
order-insensitive compare), and reference solutions run well under the time
limit on stress inputs.

Some interview staples cannot be expressed that way and are deliberately not in
the bank: class-design problems (LRU/LFU cache, iterators, "implement a data
structure supporting these operations"), problems on cyclic linked lists or on
graph-node objects (linked-list cycle detection, clone graph), interactive
problems, and problems with random output. Where a design problem has a
batch form ("process these operations, return the answers"), that form is
used instead.

## Coverage: topic × difficulty (all 1000)

| Topic | Easy | Medium | Hard | Total | Existing (1–115) | Planned (116–1000) |
|---|---:|---:|---:|---:|---:|---:|
| Arrays & Hashing | 28 | 20 | 2 | 50 | 7 | 43 |
| Two Pointers | 16 | 15 | 2 | 33 | 4 | 29 |
| Sliding Window | 5 | 20 | 6 | 31 | 4 | 27 |
| Prefix Sums & Difference Arrays | 6 | 16 | 4 | 26 | 2 | 24 |
| Sorting & Counting | 9 | 11 | 3 | 23 | 0 | 23 |
| Binary Search | 8 | 26 | 10 | 44 | 5 | 39 |
| Stacks & Monotonic Stacks/Queues | 9 | 24 | 11 | 44 | 4 | 40 |
| Linked Lists | 12 | 19 | 1 | 32 | 4 | 28 |
| Binary Trees | 31 | 26 | 6 | 63 | 11 | 52 |
| Binary Search Trees | 8 | 16 | 3 | 27 | 3 | 24 |
| Tries | 5 | 11 | 7 | 23 | 3 | 20 |
| Heaps & Top-K | 4 | 15 | 9 | 28 | 3 | 25 |
| Intervals | 4 | 14 | 4 | 22 | 3 | 19 |
| Greedy | 11 | 26 | 6 | 43 | 6 | 37 |
| Backtracking | 3 | 25 | 9 | 37 | 7 | 30 |
| Graphs: BFS/DFS & Topological Sort | 10 | 40 | 12 | 62 | 9 | 53 |
| Shortest Paths | 0 | 18 | 9 | 27 | 3 | 24 |
| MST & Union-Find | 0 | 20 | 7 | 27 | 6 | 21 |
| Advanced Graphs | 0 | 4 | 9 | 13 | 0 | 13 |
| DP: 1-D | 10 | 28 | 5 | 43 | 9 | 34 |
| DP: Grid | 3 | 14 | 5 | 22 | 3 | 19 |
| DP: Knapsack | 1 | 11 | 3 | 15 | 3 | 12 |
| DP: Strings | 0 | 13 | 8 | 21 | 4 | 17 |
| DP: Interval | 0 | 7 | 5 | 12 | 1 | 11 |
| DP: Bitmask | 0 | 3 | 8 | 11 | 0 | 11 |
| DP: Digit | 0 | 3 | 4 | 7 | 0 | 7 |
| DP: Trees | 0 | 9 | 4 | 13 | 1 | 12 |
| DP: State Machines | 2 | 5 | 5 | 12 | 0 | 12 |
| Strings: Parsing & Manipulation | 13 | 18 | 3 | 34 | 2 | 32 |
| Strings: Pattern Matching & Hashing | 5 | 10 | 3 | 18 | 0 | 18 |
| Strings: Palindromes | 5 | 8 | 4 | 17 | 3 | 14 |
| Math & Number Theory | 13 | 11 | 1 | 25 | 0 | 25 |
| Combinatorics | 1 | 6 | 4 | 11 | 0 | 11 |
| Bit Manipulation | 14 | 13 | 2 | 29 | 3 | 26 |
| Geometry | 7 | 4 | 3 | 14 | 0 | 14 |
| Matrix & Simulation | 8 | 11 | 0 | 19 | 2 | 17 |
| Range Queries (Segment/Fenwick/Sparse Table) | 0 | 5 | 7 | 12 | 0 | 12 |
| Advanced Techniques | 1 | 5 | 4 | 10 | 0 | 10 |
| **Total** | **252** | **550** | **198** | **1000** | **115** | **885** |

Existing questions are counted under the single topic that best fits them, even when they are tagged with several.

## Problems by topic

IDs 1–115 already exist in `questions/`; 116–1000 are planned. Planned IDs are contiguous by topic, so a batch of consecutive IDs is a coherent unit of work. The full spec for each planned problem (signature, compare mode, limits) lives in the plan file used to generate the questions.

### Arrays & Hashing (50)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 1 | Two Sum | easy | done |
| 3 | Best Single Stock Trade | easy | done |
| 20 | Contains Duplicate | easy | done |
| 21 | Valid Anagram | easy | done |
| 22 | Group Anagrams | medium | done |
| 23 | Longest Consecutive Sequence | medium | done |
| 27 | Majority Element | easy | done |
| 116 | Isomorphic Strings | easy | planned |
| 117 | Word Pattern | easy | planned |
| 118 | Ransom Note | easy | planned |
| 119 | First Unique Character in a String | easy | planned |
| 120 | Find the Extra Letter | easy | planned |
| 121 | Intersection of Two Arrays II | easy | planned |
| 122 | Find All Numbers Disappeared in an Array | easy | planned |
| 123 | Set Mismatch | easy | planned |
| 124 | Contains Duplicate II | easy | planned |
| 125 | Jewels and Stones | easy | planned |
| 126 | Degree of an Array | easy | planned |
| 127 | Longest Harmonious Subsequence | easy | planned |
| 128 | Unique Number of Occurrences | easy | planned |
| 129 | Minimum Index Sum of Two Lists | easy | planned |
| 130 | Kth Distinct String in an Array | easy | planned |
| 131 | Find Common Characters | easy | planned |
| 132 | Maximum Number of Balloons | easy | planned |
| 133 | X of a Kind in a Deck of Cards | easy | planned |
| 134 | Uncommon Words from Two Sentences | easy | planned |
| 135 | Number of Equivalent Domino Pairs | easy | planned |
| 136 | Max Consecutive Ones | easy | planned |
| 137 | Find Lucky Integer in an Array | easy | planned |
| 138 | Check If N and Its Double Exist | easy | planned |
| 139 | Valid Sudoku | medium | planned |
| 140 | 4Sum II | medium | planned |
| 141 | Brick Wall | medium | planned |
| 142 | Group Shifted Strings | medium | planned |
| 143 | Max Number of K-Sum Pairs | medium | planned |
| 144 | Count Number of Bad Pairs | medium | planned |
| 145 | Tuple with Same Product | medium | planned |
| 146 | Equal Row and Column Pairs | medium | planned |
| 147 | Find Players With Zero or One Losses | medium | planned |
| 148 | Determine if Two Strings Are Close | medium | planned |
| 149 | Minimum Steps to Make Two Strings Anagram | medium | planned |
| 150 | Rotate Array | medium | planned |
| 151 | Majority Element II | medium | planned |
| 152 | Find All Duplicates in an Array | medium | planned |
| 153 | Bulls and Cows | medium | planned |
| 154 | Next Permutation | medium | planned |
| 155 | Longest Square Streak in an Array | medium | planned |
| 156 | Rabbits in Forest | medium | planned |
| 157 | First Missing Positive | hard | planned |
| 158 | Count Subarrays With Median K | hard | planned |

### Two Pointers (33)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 8 | Trapped Rainwater | hard | done |
| 24 | Three Sum | medium | done |
| 25 | Container With Most Water | medium | done |
| 26 | Move Zeroes | easy | done |
| 159 | Merge Sorted Array | easy | planned |
| 160 | Remove Duplicates from Sorted Array | easy | planned |
| 161 | Remove Element | easy | planned |
| 162 | Squares of a Sorted Array | easy | planned |
| 163 | Is Subsequence | easy | planned |
| 164 | Two Sum II - Input Array Is Sorted | easy | planned |
| 165 | Backspace String Compare | easy | planned |
| 166 | Reverse Vowels of a String | easy | planned |
| 167 | Merge Strings Alternately | easy | planned |
| 168 | Long Pressed Name | easy | planned |
| 169 | Reverse Only Letters | easy | planned |
| 170 | Duplicate Zeros | easy | planned |
| 171 | Count Binary Substrings | easy | planned |
| 172 | String Compression | easy | planned |
| 173 | Valid Word Abbreviation | easy | planned |
| 174 | 3Sum Closest | medium | planned |
| 175 | 4Sum | medium | planned |
| 176 | Sort Colors | medium | planned |
| 177 | Remove Duplicates from Sorted Array II | medium | planned |
| 178 | Boats to Save People | medium | planned |
| 179 | Find the Duplicate Number | medium | planned |
| 180 | Valid Triangle Number | medium | planned |
| 181 | 3Sum With Multiplicity | medium | planned |
| 182 | Number of Subsequences That Satisfy the Given Sum Condition | medium | planned |
| 183 | Shortest Unsorted Continuous Subarray | medium | planned |
| 184 | Longest Mountain in Array | medium | planned |
| 185 | Push Dominoes | medium | planned |
| 186 | Shortest Subarray to be Removed to Make Array Sorted | medium | planned |
| 187 | Count Subarrays With Fixed Bounds | hard | planned |

### Sliding Window (31)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 4 | Longest Run of Distinct Characters | medium | done |
| 28 | Minimum Window Substring | hard | done |
| 29 | Longest Repeating Character Replacement | medium | done |
| 30 | Sliding Window Maximum | hard | done |
| 188 | Maximum Average Subarray I | easy | planned |
| 189 | Minimum Difference Between Highest and Lowest of K Scores | easy | planned |
| 190 | Defuse the Bomb | easy | planned |
| 191 | Minimum Recolors to Get K Consecutive Black Blocks | easy | planned |
| 192 | Maximum Number of Vowels in a Substring of Given Length | easy | planned |
| 193 | Permutation in String | medium | planned |
| 194 | Find All Anagrams in a String | medium | planned |
| 195 | Minimum Size Subarray Sum | medium | planned |
| 196 | Max Consecutive Ones III | medium | planned |
| 197 | Fruit Into Baskets | medium | planned |
| 198 | Longest Substring with At Most K Distinct Characters | medium | planned |
| 199 | Subarray Product Less Than K | medium | planned |
| 200 | Number of Substrings Containing All Three Characters | medium | planned |
| 201 | Count Number of Nice Subarrays | medium | planned |
| 202 | Maximum Points You Can Obtain from Cards | medium | planned |
| 203 | Get Equal Substrings Within Budget | medium | planned |
| 204 | Minimum Operations to Reduce X to Zero | medium | planned |
| 205 | Frequency of the Most Frequent Element | medium | planned |
| 206 | Minimum Swaps to Group All 1s Together II | medium | planned |
| 207 | Count Subarrays Where Max Element Appears at Least K Times | medium | planned |
| 208 | Maximum Sum of Distinct Subarrays With Length K | medium | planned |
| 209 | Longest Substring with At Least K Repeating Characters | medium | planned |
| 210 | Grumpy Bookstore Owner | medium | planned |
| 211 | Subarrays with K Different Integers | hard | planned |
| 212 | Substring with Concatenation of All Words | hard | planned |
| 213 | Minimum Number of Operations to Make Array Continuous | hard | planned |
| 214 | Minimum Adjacent Swaps for K Consecutive Ones | hard | planned |

### Prefix Sums & Difference Arrays (26)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 9 | Product of All Other Elements | medium | done |
| 100 | Subarray Sum Equals K | medium | done |
| 215 | Find Pivot Index | easy | planned |
| 216 | Range Sum Queries | easy | planned |
| 217 | Find the Highest Altitude | easy | planned |
| 218 | Minimum Value to Get Positive Step by Step Sum | easy | planned |
| 219 | Maximum Score After Splitting a String | easy | planned |
| 220 | Left and Right Sum Differences | easy | planned |
| 221 | Contiguous Array | medium | planned |
| 222 | Continuous Subarray Sum | medium | planned |
| 223 | Subarray Sums Divisible by K | medium | planned |
| 224 | Maximum Size Subarray Sum Equals k | medium | planned |
| 225 | Range Sum Query 2D | medium | planned |
| 226 | Matrix Block Sum | medium | planned |
| 227 | Car Pooling | medium | planned |
| 228 | Corporate Flight Bookings | medium | planned |
| 229 | Shifting Letters II | medium | planned |
| 230 | XOR Queries of a Subarray | medium | planned |
| 231 | Number of Wonderful Substrings | medium | planned |
| 232 | Find the Longest Substring Containing Vowels in Even Counts | medium | planned |
| 233 | Minimum Penalty for a Shop | medium | planned |
| 234 | Make Sum Divisible by P | medium | planned |
| 235 | Number of Submatrices That Sum to Target | hard | planned |
| 236 | Minimum Moves to Make Array Complementary | hard | planned |
| 237 | Minimum Number of K Consecutive Bit Flips | hard | planned |
| 238 | Find Longest Awesome Substring | hard | planned |

### Sorting & Counting (23)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 239 | Relative Sort Array | easy | planned |
| 240 | Sort Array by Increasing Frequency | easy | planned |
| 241 | Rank Transform of an Array | easy | planned |
| 242 | Height Checker | easy | planned |
| 243 | Maximum Product of Three Numbers | easy | planned |
| 244 | Largest Perimeter Triangle | easy | planned |
| 245 | Array Partition | easy | planned |
| 246 | Minimum Absolute Difference | easy | planned |
| 247 | Third Maximum Number | easy | planned |
| 248 | Sort Characters By Frequency | medium | planned |
| 249 | Custom Sort String | medium | planned |
| 250 | Largest Number | medium | planned |
| 251 | H-Index | medium | planned |
| 252 | Minimum Swaps to Sort an Array | medium | planned |
| 253 | Global and Local Inversions | medium | planned |
| 254 | Sort an Array | medium | planned |
| 255 | Sort the Matrix Diagonally | medium | planned |
| 256 | Maximum Ice Cream Bars | medium | planned |
| 257 | Find Original Array From Doubled Array | medium | planned |
| 258 | Minimum Moves to Equal Array Elements II | medium | planned |
| 259 | Maximum Gap | hard | planned |
| 260 | Orderly Queue | hard | planned |
| 261 | Minimum Cost to Make Array Equal | hard | planned |

### Binary Search (44)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 5 | Search a Rotated Sorted Array | medium | done |
| 33 | Find Minimum in Rotated Sorted Array | medium | done |
| 34 | Koko Eating Bananas | medium | done |
| 35 | Median of Two Sorted Arrays | hard | done |
| 103 | Search a 2D Matrix | medium | done |
| 262 | Binary Search | easy | planned |
| 263 | Search Insert Position | easy | planned |
| 264 | Integer Square Root | easy | planned |
| 265 | Valid Perfect Square | easy | planned |
| 266 | Arranging Coins | easy | planned |
| 267 | Count Negative Numbers in a Sorted Matrix | easy | planned |
| 268 | Find Smallest Letter Greater Than Target | easy | planned |
| 269 | Peak Index in a Mountain Array | easy | planned |
| 270 | Find First and Last Position of Element in Sorted Array | medium | planned |
| 271 | Search in Rotated Sorted Array II | medium | planned |
| 272 | Single Element in a Sorted Array | medium | planned |
| 273 | Search a 2D Matrix II | medium | planned |
| 274 | Capacity to Ship Packages Within D Days | medium | planned |
| 275 | Minimum Number of Days to Make m Bouquets | medium | planned |
| 276 | Find the Smallest Divisor Given a Threshold | medium | planned |
| 277 | Kth Smallest Element in a Sorted Matrix | medium | planned |
| 278 | Find K Closest Elements | medium | planned |
| 279 | H-Index II | medium | planned |
| 280 | Successful Pairs of Spells and Potions | medium | planned |
| 281 | Magnetic Force Between Two Balls | medium | planned |
| 282 | Minimized Maximum of Products Distributed to Any Store | medium | planned |
| 283 | Minimum Time to Complete Trips | medium | planned |
| 284 | Maximum Candies Allocated to K Children | medium | planned |
| 285 | Maximum Value at a Given Index in a Bounded Array | medium | planned |
| 286 | Minimum Speed to Arrive on Time | medium | planned |
| 287 | Heaters | medium | planned |
| 288 | Sum of Mutated Array Closest to Target | medium | planned |
| 289 | Maximum Number of Removable Characters | medium | planned |
| 290 | Minimum Limit of Balls in a Bag | medium | planned |
| 291 | Ugly Number III | medium | planned |
| 292 | Split Array Largest Sum | hard | planned |
| 293 | Kth Smallest Number in Multiplication Table | hard | planned |
| 294 | Find K-th Smallest Pair Distance | hard | planned |
| 295 | Find Minimum in Rotated Sorted Array II | hard | planned |
| 296 | Nth Magical Number | hard | planned |
| 297 | Minimize Max Distance to Gas Station | hard | planned |
| 298 | Maximum Running Time of N Computers | hard | planned |
| 299 | Kth Smallest Product of Two Sorted Arrays | hard | planned |
| 300 | Maximize the Minimum Powered City | hard | planned |

### Stacks & Monotonic Stacks/Queues (44)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 2 | Balanced Brackets | easy | done |
| 31 | Daily Temperatures | medium | done |
| 32 | Largest Rectangle in Histogram | hard | done |
| 111 | Evaluate Reverse Polish Notation | medium | done |
| 301 | Baseball Game | easy | planned |
| 302 | Remove All Adjacent Duplicates in String | easy | planned |
| 303 | Next Greater Element I | easy | planned |
| 304 | Make the String Great | easy | planned |
| 305 | Final Prices With a Special Discount in a Shop | easy | planned |
| 306 | Crawler Log Folder | easy | planned |
| 307 | Maximum Nesting Depth of the Parentheses | easy | planned |
| 308 | Remove Outermost Parentheses | easy | planned |
| 309 | Decode String | medium | planned |
| 310 | Simplify Path | medium | planned |
| 311 | Remove K Digits | medium | planned |
| 312 | Asteroid Collision | medium | planned |
| 313 | Next Greater Element II | medium | planned |
| 314 | Stock Span | medium | planned |
| 315 | Sum of Subarray Minimums | medium | planned |
| 316 | Basic Calculator II | medium | planned |
| 317 | Remove Duplicate Letters | medium | planned |
| 318 | Validate Stack Sequences | medium | planned |
| 319 | Score of Parentheses | medium | planned |
| 320 | Minimum Remove to Make Valid Parentheses | medium | planned |
| 321 | Minimum Add to Make Parentheses Valid | medium | planned |
| 322 | 132 Pattern | medium | planned |
| 323 | Car Fleet | medium | planned |
| 324 | Remove All Adjacent Duplicates in String II | medium | planned |
| 325 | Sum of Subarray Ranges | medium | planned |
| 326 | Longest Continuous Subarray With Absolute Diff Within Limit | medium | planned |
| 327 | Exclusive Time of Functions | medium | planned |
| 328 | Next Greater Node in Linked List | medium | planned |
| 329 | Maximum Width Ramp | medium | planned |
| 330 | Reverse Substrings Between Each Pair of Parentheses | medium | planned |
| 331 | Basic Calculator | hard | planned |
| 332 | Maximal Rectangle | hard | planned |
| 333 | Shortest Subarray with Sum at Least K | hard | planned |
| 334 | Number of Visible People in a Queue | hard | planned |
| 335 | Longest Valid Parentheses | hard | planned |
| 336 | Max Chunks To Make Sorted II | hard | planned |
| 337 | Sum of Total Strength of Wizards | hard | planned |
| 338 | Number of Atoms | hard | planned |
| 339 | Odd Even Jump | hard | planned |
| 340 | Maximum Score of a Good Subarray | hard | planned |

### Linked Lists (32)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 6 | Reverse a Linked List | easy | done |
| 36 | Merge Two Sorted Lists | easy | done |
| 37 | Remove Nth Node From End of List | medium | done |
| 38 | Add Two Numbers | medium | done |
| 341 | Middle of the Linked List | easy | planned |
| 342 | Palindrome Linked List | easy | planned |
| 343 | Remove Linked List Elements | easy | planned |
| 344 | Remove Duplicates from Sorted List | easy | planned |
| 345 | Binary Number in a Linked List to Integer | easy | planned |
| 346 | Merge Nodes in Between Zeros | easy | planned |
| 347 | Maximum Twin Sum of a Linked List | easy | planned |
| 348 | Delete the Middle Node of a Linked List | easy | planned |
| 349 | Insert Greatest Common Divisors in a Linked List | easy | planned |
| 350 | Delete N Nodes After M Nodes of a Linked List | easy | planned |
| 351 | Swap Nodes in Pairs | medium | planned |
| 352 | Rotate List | medium | planned |
| 353 | Partition List | medium | planned |
| 354 | Odd Even Linked List | medium | planned |
| 355 | Reverse Linked List II | medium | planned |
| 356 | Reorder List | medium | planned |
| 357 | Sort List | medium | planned |
| 358 | Insertion Sort List | medium | planned |
| 359 | Remove Duplicates from Sorted List II | medium | planned |
| 360 | Add Two Numbers II | medium | planned |
| 361 | Split Linked List in Parts | medium | planned |
| 362 | Swapping Nodes in a Linked List | medium | planned |
| 363 | Linked List Components | medium | planned |
| 364 | Remove Zero Sum Consecutive Nodes from Linked List | medium | planned |
| 365 | Double a Number Represented as a Linked List | medium | planned |
| 366 | Remove Nodes From Linked List | medium | planned |
| 367 | Merge In Between Linked Lists | medium | planned |
| 368 | Reverse Nodes in k-Group | hard | planned |

### Binary Trees (63)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 10 | Maximum Depth of a Binary Tree | easy | done |
| 17 | Binary Tree Level Order Traversal | medium | done |
| 39 | Invert Binary Tree | easy | done |
| 40 | Same Tree | easy | done |
| 69 | Binary Tree Right Side View | medium | done |
| 70 | Construct Binary Tree from Preorder and Inorder Traversal | medium | done |
| 71 | Diameter of a Binary Tree | easy | done |
| 72 | Balanced Binary Tree | easy | done |
| 73 | Path Sum II | medium | done |
| 74 | Subtree of Another Tree | easy | done |
| 75 | Count Good Nodes in a Binary Tree | medium | done |
| 369 | Binary Tree Inorder Traversal | easy | planned |
| 370 | Binary Tree Postorder Traversal | easy | planned |
| 371 | Symmetric Tree | easy | planned |
| 372 | Minimum Depth of Binary Tree | easy | planned |
| 373 | Path Sum | easy | planned |
| 374 | Merge Two Binary Trees | easy | planned |
| 375 | Sum of Left Leaves | easy | planned |
| 376 | Binary Tree Paths | easy | planned |
| 377 | Average of Levels in Binary Tree | easy | planned |
| 378 | Univalued Binary Tree | easy | planned |
| 379 | Cousins in Binary Tree | easy | planned |
| 380 | Leaf-Similar Trees | easy | planned |
| 381 | Sum of Root To Leaf Binary Numbers | easy | planned |
| 382 | Second Minimum Node in a Binary Tree | easy | planned |
| 383 | Evaluate Boolean Binary Tree | easy | planned |
| 384 | Binary Tree Tilt | easy | planned |
| 385 | Construct String from Binary Tree | easy | planned |
| 386 | Find Bottom Left Tree Value | easy | planned |
| 387 | Find Largest Value in Each Tree Row | easy | planned |
| 388 | Deepest Leaves Sum | easy | planned |
| 389 | Sum of Nodes with Even-Valued Grandparent | easy | planned |
| 390 | Count Nodes Equal to Average of Subtree | easy | planned |
| 391 | Even Odd Tree | easy | planned |
| 392 | Binary Tree Preorder Traversal | easy | planned |
| 393 | Count Complete Tree Nodes | easy | planned |
| 394 | Binary Tree Zigzag Level Order Traversal | medium | planned |
| 395 | Maximum Width of Binary Tree | medium | planned |
| 396 | Lowest Common Ancestor of a Binary Tree | medium | planned |
| 397 | Flatten Binary Tree to Linked List | medium | planned |
| 398 | Construct Binary Tree from Inorder and Postorder Traversal | medium | planned |
| 399 | Construct Binary Tree from Preorder and Postorder Traversal | medium | planned |
| 400 | Path Sum III | medium | planned |
| 401 | All Nodes Distance K in Binary Tree | medium | planned |
| 402 | Find Duplicate Subtrees | medium | planned |
| 403 | Delete Nodes And Return Forest | medium | planned |
| 404 | Boundary of Binary Tree | medium | planned |
| 405 | Find Leaves of Binary Tree | medium | planned |
| 406 | Smallest Subtree with all the Deepest Nodes | medium | planned |
| 407 | Maximum Difference Between Node and Ancestor | medium | planned |
| 408 | Check Completeness of a Binary Tree | medium | planned |
| 409 | Step-By-Step Directions From a Binary Tree Node to Another | medium | planned |
| 410 | Create Binary Tree From Descriptions | medium | planned |
| 411 | Amount of Time for Binary Tree to Be Infected | medium | planned |
| 412 | Serialize and Deserialize Binary Tree | medium | planned |
| 413 | Sum Root to Leaf Numbers | medium | planned |
| 414 | Maximum Binary Tree | medium | planned |
| 415 | Vertical Order Traversal of a Binary Tree | hard | planned |
| 416 | Recover a Tree From Preorder Traversal | hard | planned |
| 417 | Height of Binary Tree After Subtree Removal Queries | hard | planned |
| 418 | Cycle Length Queries in a Tree | hard | planned |
| 419 | Count Paths That Can Form a Palindrome in a Tree | hard | planned |
| 420 | Lowest Common Ancestor Queries | hard | planned |

### Binary Search Trees (27)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 41 | Validate Binary Search Tree | medium | done |
| 42 | Lowest Common Ancestor of a BST | medium | done |
| 68 | Kth Smallest Element in a BST | medium | done |
| 421 | Search in a Binary Search Tree | easy | planned |
| 422 | Range Sum of BST | easy | planned |
| 423 | Convert Sorted Array to Binary Search Tree | easy | planned |
| 424 | Minimum Absolute Difference in BST | easy | planned |
| 425 | Two Sum IV - Input is a BST | easy | planned |
| 426 | Find Mode in Binary Search Tree | easy | planned |
| 427 | Increasing Order Search Tree | easy | planned |
| 428 | Closest Binary Search Tree Value | easy | planned |
| 429 | Insert into a Binary Search Tree | medium | planned |
| 430 | Delete Node in a BST | medium | planned |
| 431 | Trim a Binary Search Tree | medium | planned |
| 432 | Convert Sorted List to Binary Search Tree | medium | planned |
| 433 | Recover Binary Search Tree | medium | planned |
| 434 | Inorder Successor in BST | medium | planned |
| 435 | Construct Binary Search Tree from Preorder Traversal | medium | planned |
| 436 | Unique Binary Search Trees II | medium | planned |
| 437 | Balance a Binary Search Tree | medium | planned |
| 438 | Convert BST to Greater Tree | medium | planned |
| 439 | All Elements in Two Binary Search Trees | medium | planned |
| 440 | Closest Nodes Queries in a Binary Search Tree | medium | planned |
| 441 | Verify Preorder Sequence in Binary Search Tree | medium | planned |
| 442 | Number of Ways to Reorder Array to Get Same BST | hard | planned |
| 443 | Closest Binary Search Tree Value II | hard | planned |
| 444 | Merge BSTs to Create Single BST | hard | planned |

### Tries (23)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 76 | Word Search II | hard | done |
| 77 | Replace Words | medium | done |
| 78 | Longest Word in Dictionary | medium | done |
| 445 | Counting Words With a Given Prefix | easy | planned |
| 446 | Check If a Word Occurs As a Prefix of Any Word in a Sentence | easy | planned |
| 447 | Index Pairs of a String | easy | planned |
| 448 | Count Prefixes of a Given String | easy | planned |
| 449 | Count Prefix and Suffix Pairs | easy | planned |
| 450 | Search Suggestions System | medium | planned |
| 451 | Maximum XOR of Two Numbers in an Array | medium | planned |
| 452 | Short Encoding of Words | medium | planned |
| 453 | Find the Length of the Longest Common Prefix | medium | planned |
| 454 | Shortest Unique Prefixes | medium | planned |
| 455 | Lexicographical Numbers | medium | planned |
| 456 | Camelcase Matching | medium | planned |
| 457 | Remove Sub-Folders from the Filesystem | medium | planned |
| 458 | Longest Common Suffix Queries | medium | planned |
| 459 | Concatenated Words | hard | planned |
| 460 | Maximum XOR With an Element From Array | hard | planned |
| 461 | Stream of Characters | hard | planned |
| 462 | K-th Smallest in Lexicographical Order | hard | planned |
| 463 | Sum of Prefix Scores of Strings | hard | planned |
| 464 | Word Squares | hard | planned |

### Heaps & Top-K (28)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 15 | Top K Frequent Elements | medium | done |
| 19 | Merge K Sorted Lists | hard | done |
| 60 | Kth Largest Element in an Array | medium | done |
| 465 | Last Stone Weight | easy | planned |
| 466 | Kth Largest in a Stream | easy | planned |
| 467 | The K Weakest Rows in a Matrix | easy | planned |
| 468 | Take Gifts From the Richest Pile | easy | planned |
| 469 | K Closest Points to Origin | medium | planned |
| 470 | Find K Pairs with Smallest Sums | medium | planned |
| 471 | Top K Frequent Words | medium | planned |
| 472 | Reorganize String | medium | planned |
| 473 | Minimum Cost to Connect Sticks | medium | planned |
| 474 | Running Median of a Stream | medium | planned |
| 475 | Merge K Sorted Arrays | medium | planned |
| 476 | Process Tasks Using Servers | medium | planned |
| 477 | Single-Threaded CPU | medium | planned |
| 478 | Furthest Building You Can Reach | medium | planned |
| 479 | Maximum Subsequence Score | medium | planned |
| 480 | Total Cost to Hire K Workers | medium | planned |
| 481 | K-th Smallest Prime Fraction | medium | planned |
| 482 | Smallest Range Covering Elements from K Lists | hard | planned |
| 483 | IPO | hard | planned |
| 484 | Sliding Window Median | hard | planned |
| 485 | The Skyline Problem | hard | planned |
| 486 | Minimum Cost to Hire K Workers | hard | planned |
| 487 | Trapping Rain Water II | hard | planned |
| 488 | Maximum Performance of a Team | hard | planned |
| 489 | Meeting Rooms III | hard | planned |

### Intervals (22)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 7 | Merge Overlapping Intervals | medium | done |
| 62 | Meeting Rooms II | medium | done |
| 63 | Non-overlapping Intervals | medium | done |
| 490 | Meeting Rooms | easy | planned |
| 491 | Summary Ranges | easy | planned |
| 492 | Missing Ranges | easy | planned |
| 493 | Teemo Attacking | easy | planned |
| 494 | Insert Interval | medium | planned |
| 495 | Interval List Intersections | medium | planned |
| 496 | Remove Covered Intervals | medium | planned |
| 497 | Minimum Number of Arrows to Burst Balloons | medium | planned |
| 498 | My Calendar I | medium | planned |
| 499 | My Calendar II | medium | planned |
| 500 | Remove Interval | medium | planned |
| 501 | Find Right Interval | medium | planned |
| 502 | Count Days Without Meetings | medium | planned |
| 503 | Maximum Number of Events That Can Be Attended | medium | planned |
| 504 | Video Stitching | medium | planned |
| 505 | Minimum Interval to Include Each Query | hard | planned |
| 506 | Employee Free Time | hard | planned |
| 507 | My Calendar III | hard | planned |
| 508 | Set Intersection Size At Least Two | hard | planned |

### Greedy (43)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 61 | Task Scheduler | medium | done |
| 106 | Jump Game | medium | done |
| 107 | Jump Game II | medium | done |
| 113 | Gas Station | medium | done |
| 114 | Hand of Straights | medium | done |
| 115 | Partition Labels | medium | done |
| 509 | Assign Cookies | easy | planned |
| 510 | Lemonade Change | easy | planned |
| 511 | Maximum Units on a Truck | easy | planned |
| 512 | Minimum Moves to Seat Everyone | easy | planned |
| 513 | Can Place Flowers | easy | planned |
| 514 | Maximum 69 Number | easy | planned |
| 515 | Split a String in Balanced Strings | easy | planned |
| 516 | Minimum Cost of Buying Candies With Discount | easy | planned |
| 517 | Maximize Sum Of Array After K Negations | easy | planned |
| 518 | Longest Palindrome | easy | planned |
| 519 | Largest Odd Number in String | easy | planned |
| 520 | Two City Scheduling | medium | planned |
| 521 | Queue Reconstruction by Height | medium | planned |
| 522 | Wiggle Subsequence | medium | planned |
| 523 | Valid Parenthesis String | medium | planned |
| 524 | Broken Calculator | medium | planned |
| 525 | Monotone Increasing Digits | medium | planned |
| 526 | Bag of Tokens | medium | planned |
| 527 | Advantage Shuffle | medium | planned |
| 528 | Minimum Deletions to Make Character Frequencies Unique | medium | planned |
| 529 | Increasing Triplet Subsequence | medium | planned |
| 530 | Maximum Swap | medium | planned |
| 531 | Score After Flipping Matrix | medium | planned |
| 532 | Minimum Domino Rotations For Equal Row | medium | planned |
| 533 | Optimal Partition of String | medium | planned |
| 534 | Reduce Array Size to The Half | medium | planned |
| 535 | Eliminate Maximum Number of Monsters | medium | planned |
| 536 | Dota2 Senate | medium | planned |
| 537 | Partition Array Such That Maximum Difference Is K | medium | planned |
| 538 | Minimum Increment to Make Array Unique | medium | planned |
| 539 | Smallest String With A Given Numeric Value | medium | planned |
| 540 | Minimum Number of Taps to Open to Water a Garden | hard | planned |
| 541 | Candy | hard | planned |
| 542 | Minimum Number of Refueling Stops | hard | planned |
| 543 | Patching Array | hard | planned |
| 544 | Course Schedule III | hard | planned |
| 545 | Minimum Number of Increments on Subarrays to Form a Target Array | hard | planned |

### Backtracking (37)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 16 | Subsets | medium | done |
| 64 | Permutations | medium | done |
| 65 | Combination Sum | medium | done |
| 66 | Word Search | medium | done |
| 67 | N-Queens II | hard | done |
| 79 | Palindrome Partitioning | medium | done |
| 112 | Generate Parentheses | medium | done |
| 546 | Letter Case Permutation | easy | planned |
| 547 | Binary Watch | easy | planned |
| 548 | Sum of All Subset XOR Totals | easy | planned |
| 549 | Subsets II | medium | planned |
| 550 | Permutations II | medium | planned |
| 551 | Combinations | medium | planned |
| 552 | Combination Sum II | medium | planned |
| 553 | Combination Sum III | medium | planned |
| 554 | Letter Combinations of a Phone Number | medium | planned |
| 555 | Restore IP Addresses | medium | planned |
| 556 | Matchsticks to Square | medium | planned |
| 557 | Partition to K Equal Sum Subsets | medium | planned |
| 558 | Beautiful Arrangement | medium | planned |
| 559 | Split a String into Descending Consecutive Values | medium | planned |
| 560 | Non-decreasing Subsequences | medium | planned |
| 561 | Longest Concatenation with Unique Characters | medium | planned |
| 562 | Path with Maximum Gold | medium | planned |
| 563 | Palindrome Permutations II | medium | planned |
| 564 | Numbers With Same Consecutive Differences | medium | planned |
| 565 | Split Array into Fibonacci Sequence | medium | planned |
| 566 | Letter Tile Possibilities | medium | planned |
| 567 | Brace Expansion | medium | planned |
| 568 | N-Queens | hard | planned |
| 569 | Sudoku Solver | hard | planned |
| 570 | Word Break II | hard | planned |
| 571 | Expression Add Operators | hard | planned |
| 572 | Remove Invalid Parentheses | hard | planned |
| 573 | Unique Paths III | hard | planned |
| 574 | Maximum Score Words Formed by Letters | hard | planned |
| 575 | 24 Game | hard | planned |

### Graphs: BFS/DFS & Topological Sort (62)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 11 | Number of Islands | medium | done |
| 12 | Course Schedule | medium | done |
| 46 | Rotting Oranges | medium | done |
| 47 | Pacific Atlantic Water Flow | medium | done |
| 48 | Word Ladder | hard | done |
| 51 | Course Schedule II | medium | done |
| 94 | Surrounded Regions | medium | done |
| 95 | Walls and Gates | medium | done |
| 98 | Alien Dictionary | hard | done |
| 576 | Flood Fill | easy | planned |
| 577 | Island Perimeter | easy | planned |
| 578 | Find if Path Exists in Graph | easy | planned |
| 579 | Find the Town Judge | easy | planned |
| 580 | Find Center of Star Graph | easy | planned |
| 581 | Destination City | easy | planned |
| 582 | Keys and Rooms | easy | planned |
| 583 | Shortest Hops from a Source | easy | planned |
| 584 | Find the Champion in a Tournament DAG | easy | planned |
| 585 | Maximal Network Rank | easy | planned |
| 586 | Max Area of Island | medium | planned |
| 587 | Number of Closed Islands | medium | planned |
| 588 | Number of Enclaves | medium | planned |
| 589 | Count Sub Islands | medium | planned |
| 590 | Number of Distinct Islands | medium | planned |
| 591 | Find All Groups of Farmland | medium | planned |
| 592 | Shortest Path in Binary Matrix | medium | planned |
| 593 | As Far from Land as Possible | medium | planned |
| 594 | 01 Matrix | medium | planned |
| 595 | Nearest Exit from Entrance in Maze | medium | planned |
| 596 | Open the Lock | medium | planned |
| 597 | Snakes and Ladders | medium | planned |
| 598 | Minimum Genetic Mutation | medium | planned |
| 599 | Minimum Knight Moves | medium | planned |
| 600 | Shortest Bridge | medium | planned |
| 601 | Jump Game III | medium | planned |
| 602 | Is Graph Bipartite? | medium | planned |
| 603 | Possible Bipartition | medium | planned |
| 604 | Find Eventual Safe States | medium | planned |
| 605 | Minimum Height Trees | medium | planned |
| 606 | Course Schedule IV | medium | planned |
| 607 | Parallel Courses | medium | planned |
| 608 | Parallel Courses III | medium | planned |
| 609 | All Ancestors of a Node in a DAG | medium | planned |
| 610 | Loud and Rich | medium | planned |
| 611 | Time Needed to Inform All Employees | medium | planned |
| 612 | Reorder Routes to Make All Paths Lead to City Zero | medium | planned |
| 613 | All Paths From Source to Target | medium | planned |
| 614 | Evaluate Division | medium | planned |
| 615 | Detonate the Maximum Bombs | medium | planned |
| 616 | Count Unreachable Pairs of Nodes | medium | planned |
| 617 | Detect Cycles in 2D Grid | medium | planned |
| 618 | Minimum Fuel Cost to Report to the Capital | medium | planned |
| 619 | Making a Large Island | hard | planned |
| 620 | Bus Routes | hard | planned |
| 621 | Shortest Path to Get All Keys | hard | planned |
| 622 | Sliding Puzzle | hard | planned |
| 623 | Jump Game IV | hard | planned |
| 624 | Word Ladder II | hard | planned |
| 625 | Shortest Path in a Grid with Obstacles Elimination | hard | planned |
| 626 | Minimum Moves to Move a Box to Their Target Location | hard | planned |
| 627 | Largest Color Value in a Directed Graph | hard | planned |
| 628 | Escape the Spreading Fire | hard | planned |

### Shortest Paths (27)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 49 | Network Delay Time | medium | done |
| 50 | Cheapest Flights Within K Stops | medium | done |
| 97 | Swim in Rising Water | hard | done |
| 629 | Shortest Paths with Negative Edges | medium | planned |
| 630 | All-Pairs Shortest Paths | medium | planned |
| 631 | Path With Minimum Effort | medium | planned |
| 632 | Path with Maximum Probability | medium | planned |
| 633 | Find the City With the Smallest Number of Neighbors at a Threshold Distance | medium | planned |
| 634 | Number of Ways to Arrive at Destination | medium | planned |
| 635 | The Maze II | medium | planned |
| 636 | Minimum Obstacle Removal to Reach Corner | medium | planned |
| 637 | Minimum Cost to Make at Least One Valid Path in a Grid | medium | planned |
| 638 | Path With Maximum Minimum Value | medium | planned |
| 639 | Minimum Cost of a Path With Special Roads | medium | planned |
| 640 | Shortest Path with Alternating Colors | medium | planned |
| 641 | Minimum Cost to Convert String | medium | planned |
| 642 | Number of Restricted Paths From First to Last Node | medium | planned |
| 643 | Minimum Cost to Reach City With Discounts | medium | planned |
| 644 | Find the Safest Path in a Grid | medium | planned |
| 645 | The Maze III | hard | planned |
| 646 | Reachable Nodes In Subdivided Graph | hard | planned |
| 647 | Minimum Time to Visit a Cell In a Grid | hard | planned |
| 648 | Minimum Cost to Reach Destination in Time | hard | planned |
| 649 | Second Minimum Time to Reach Destination | hard | planned |
| 650 | Find Edges in Shortest Paths | hard | planned |
| 651 | Minimum Weighted Subgraph With the Required Paths | hard | planned |
| 652 | Shortest Cycle in a Graph | hard | planned |

### MST & Union-Find (27)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 44 | Number of Connected Components | medium | done |
| 45 | Redundant Connection | medium | done |
| 92 | Number of Provinces | medium | done |
| 93 | Graph Valid Tree | medium | done |
| 96 | Min Cost to Connect All Points | medium | done |
| 99 | Accounts Merge | medium | done |
| 653 | Connecting Cities With Minimum Cost | medium | planned |
| 654 | Number of Operations to Make Network Connected | medium | planned |
| 655 | Satisfiability of Equality Equations | medium | planned |
| 656 | Lexicographically Smallest Equivalent String | medium | planned |
| 657 | Smallest String With Swaps | medium | planned |
| 658 | Most Stones Removed with Same Row or Column | medium | planned |
| 659 | Similar String Groups | medium | planned |
| 660 | Regions Cut By Slashes | medium | planned |
| 661 | The Earliest Moment When Everyone Become Friends | medium | planned |
| 662 | Count the Number of Complete Components | medium | planned |
| 663 | Minimum Score of a Path Between Two Cities | medium | planned |
| 664 | Number of Islands II | medium | planned |
| 665 | Couples Holding Hands | medium | planned |
| 666 | Optimize Water Distribution in a Village | medium | planned |
| 667 | Largest Component Size by Common Factor | hard | planned |
| 668 | Rank Transform of a Matrix | hard | planned |
| 669 | Minimize Malware Spread | hard | planned |
| 670 | Find Critical and Pseudo-Critical Edges in Minimum Spanning Tree | hard | planned |
| 671 | Checking Existence of Edge Length Limited Paths | hard | planned |
| 672 | Number of Good Paths | hard | planned |
| 673 | Remove Max Number of Edges to Keep Graph Fully Traversable | hard | planned |

### Advanced Graphs (13)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 674 | Draw Without Lifting the Pen | medium | planned |
| 675 | Longest Cycle in a Graph | medium | planned |
| 676 | Kth Ancestor Queries | medium | planned |
| 677 | Minimum Number of Vertices to Reach All Nodes | medium | planned |
| 678 | Reconstruct Itinerary | hard | planned |
| 679 | Valid Arrangement of Pairs | hard | planned |
| 680 | Cracking the Safe | hard | planned |
| 681 | Critical Connections in a Network | hard | planned |
| 682 | Articulation Points | hard | planned |
| 683 | Strongly Connected Components | hard | planned |
| 684 | Maximum Flow | hard | planned |
| 685 | Maximum Bipartite Matching | hard | planned |
| 686 | Minimum Number of Days to Disconnect Island | hard | planned |

### DP: 1-D (43)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 13 | Climbing Stairs | easy | done |
| 14 | Coin Change | medium | done |
| 18 | Longest Increasing Subsequence | medium | done |
| 52 | House Robber | medium | done |
| 53 | House Robber II | medium | done |
| 54 | Decode Ways | medium | done |
| 55 | Word Break | medium | done |
| 101 | Maximum Subarray | medium | done |
| 102 | Maximum Product Subarray | medium | done |
| 687 | Min Cost Climbing Stairs | easy | planned |
| 688 | N-th Tribonacci Number | easy | planned |
| 689 | Divisor Game | easy | planned |
| 690 | Get Maximum in Generated Array | easy | planned |
| 691 | Longest Continuous Increasing Subsequence | easy | planned |
| 692 | Count Sorted Vowel Strings | easy | planned |
| 693 | Paint Fence | easy | planned |
| 694 | Best Sightseeing Pair | easy | planned |
| 695 | Arithmetic Slices | easy | planned |
| 696 | Delete and Earn | medium | planned |
| 697 | Perfect Squares | medium | planned |
| 698 | Integer Break | medium | planned |
| 699 | Number of Longest Increasing Subsequence | medium | planned |
| 700 | Largest Divisible Subset | medium | planned |
| 701 | Longest Arithmetic Subsequence | medium | planned |
| 702 | Longest Arithmetic Subsequence of Given Difference | medium | planned |
| 703 | Longest String Chain | medium | planned |
| 704 | Maximum Sum Circular Subarray | medium | planned |
| 705 | Longest Turbulent Subarray | medium | planned |
| 706 | Maximum Subarray Sum with One Deletion | medium | planned |
| 707 | Minimum Cost For Tickets | medium | planned |
| 708 | Solving Questions With Brainpower | medium | planned |
| 709 | Count Ways To Build Good Strings | medium | planned |
| 710 | Domino and Tromino Tiling | medium | planned |
| 711 | Maximum Alternating Subsequence Sum | medium | planned |
| 712 | Partition Array for Maximum Sum | medium | planned |
| 713 | Frog Jump | medium | planned |
| 714 | Maximum Sum of 3 Non-Overlapping Subarrays | medium | planned |
| 715 | Filling Bookcase Shelves | medium | planned |
| 716 | Decode Ways II | hard | planned |
| 717 | Russian Doll Envelopes | hard | planned |
| 718 | Minimum Difficulty of a Job Schedule | hard | planned |
| 719 | Maximum Profit in Job Scheduling | hard | planned |
| 720 | Number of Ways to Stay in the Same Place After Some Steps | hard | planned |

### DP: Grid (22)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 58 | Unique Paths | medium | done |
| 87 | Longest Increasing Path in a Matrix | hard | done |
| 88 | Maximal Square | medium | done |
| 721 | Minimum Path Sum | easy | planned |
| 722 | Triangle | easy | planned |
| 723 | Minimum Falling Path Sum | easy | planned |
| 724 | Unique Paths II | medium | planned |
| 725 | Count Square Submatrices with All Ones | medium | planned |
| 726 | Out of Boundary Paths | medium | planned |
| 727 | Knight Probability in Chessboard | medium | planned |
| 728 | Knight Dialer | medium | planned |
| 729 | Largest Plus Sign | medium | planned |
| 730 | Longest Line of Consecutive One in Matrix | medium | planned |
| 731 | Maximum Non Negative Product in a Matrix | medium | planned |
| 732 | Champagne Tower | medium | planned |
| 733 | Minimum Falling Path Sum II | medium | planned |
| 734 | Number of Paths with Max Score | medium | planned |
| 735 | Paths in Matrix Whose Sum Is Divisible by K | medium | planned |
| 736 | Dungeon Game | hard | planned |
| 737 | Cherry Pickup | hard | planned |
| 738 | Cherry Pickup II | hard | planned |
| 739 | Number of Ways of Cutting a Pizza | hard | planned |

### DP: Knapsack (15)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 84 | Partition Equal Subset Sum | medium | done |
| 85 | Target Sum | medium | done |
| 86 | Coin Change II | medium | done |
| 740 | 0/1 Knapsack | easy | planned |
| 741 | Last Stone Weight II | medium | planned |
| 742 | Ones and Zeroes | medium | planned |
| 743 | Combination Sum IV | medium | planned |
| 744 | Number of Dice Rolls With Target Sum | medium | planned |
| 745 | Unbounded Knapsack | medium | planned |
| 746 | Number of Ways to Earn Points | medium | planned |
| 747 | Shopping Offers | medium | planned |
| 748 | Maximum Value of K Coins From Piles | medium | planned |
| 749 | Profitable Schemes | hard | planned |
| 750 | Tallest Billboard | hard | planned |
| 751 | Form Largest Integer With Digits That Add up to Target | hard | planned |

### DP: Strings (21)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 56 | Longest Common Subsequence | medium | done |
| 57 | Edit Distance | hard | done |
| 90 | Regular Expression Matching | hard | done |
| 91 | Interleaving String | medium | done |
| 752 | Longest Palindromic Subsequence | medium | planned |
| 753 | Minimum Insertion Steps to Make a String Palindrome | medium | planned |
| 754 | Delete Operation for Two Strings | medium | planned |
| 755 | Minimum ASCII Delete Sum for Two Strings | medium | planned |
| 756 | Uncrossed Lines | medium | planned |
| 757 | Maximum Length of Repeated Subarray | medium | planned |
| 758 | Flip String to Monotone Increasing | medium | planned |
| 759 | Longest Ideal Subsequence | medium | planned |
| 760 | Unique Substrings in Wraparound String | medium | planned |
| 761 | Palindrome Partitioning II | medium | planned |
| 762 | Max Dot Product of Two Subsequences | medium | planned |
| 763 | Distinct Subsequences | hard | planned |
| 764 | Wildcard Matching | hard | planned |
| 765 | Shortest Common Supersequence | hard | planned |
| 766 | Scramble String | hard | planned |
| 767 | Number of Ways to Form a Target String Given a Dictionary | hard | planned |
| 768 | Minimum Window Subsequence | hard | planned |

### DP: Interval (12)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 89 | Burst Balloons | hard | done |
| 769 | Minimum Cost Tree From Leaf Values | medium | planned |
| 770 | Predict the Winner | medium | planned |
| 771 | Stone Game II | medium | planned |
| 772 | Stone Game III | medium | planned |
| 773 | Stone Game VII | medium | planned |
| 774 | Minimum Score Triangulation of Polygon | medium | planned |
| 775 | Guess Number Higher or Lower II | medium | planned |
| 776 | Strange Printer | hard | planned |
| 777 | Remove Boxes | hard | planned |
| 778 | Minimum Cost to Cut a Stick | hard | planned |
| 779 | Minimum Cost to Merge Stones | hard | planned |

### DP: Bitmask (11)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 780 | Can I Win | medium | planned |
| 781 | Minimum Number of Work Sessions to Finish the Tasks | medium | planned |
| 782 | Travelling Salesman Tour | medium | planned |
| 783 | Shortest Path Visiting All Nodes | hard | planned |
| 784 | Find the Shortest Superstring | hard | planned |
| 785 | Minimum Incompatibility | hard | planned |
| 786 | Parallel Courses II | hard | planned |
| 787 | Maximum Students Taking Exam | hard | planned |
| 788 | Number of Ways to Wear Different Hats to Each Other | hard | planned |
| 789 | Smallest Sufficient Team | hard | planned |
| 790 | Find Minimum Time to Finish All Jobs | hard | planned |

### DP: Digit (7)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 791 | Count Numbers with Unique Digits | medium | planned |
| 792 | Rotated Digits | medium | planned |
| 793 | Numbers At Most N Given Digit Set | medium | planned |
| 794 | Non-negative Integers without Consecutive Ones | hard | planned |
| 795 | Number of Digit One | hard | planned |
| 796 | Count of Integers With Digit Sum in Range | hard | planned |
| 797 | Numbers With Repeated Digits | hard | planned |

### DP: Trees (13)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 43 | Binary Tree Maximum Path Sum | hard | done |
| 798 | House Robber III | medium | planned |
| 799 | Distribute Coins in Binary Tree | medium | planned |
| 800 | Longest Univalue Path | medium | planned |
| 801 | Maximum Product of Splitted Binary Tree | medium | planned |
| 802 | Longest ZigZag Path in a Binary Tree | medium | planned |
| 803 | Minimum Time to Collect All Apples in a Tree | medium | planned |
| 804 | Unique Binary Search Trees | medium | planned |
| 805 | Tree Diameter | medium | planned |
| 806 | Longest Path With Different Adjacent Characters | medium | planned |
| 807 | Binary Tree Cameras | hard | planned |
| 808 | Sum of Distances in Tree | hard | planned |
| 809 | Maximum Sum BST in Binary Tree | hard | planned |

### DP: State Machines (12)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 810 | Best Time to Buy and Sell Stock II | easy | planned |
| 811 | Paint House | easy | planned |
| 812 | Best Time to Buy and Sell Stock with Cooldown | medium | planned |
| 813 | Best Time to Buy and Sell Stock with Transaction Fee | medium | planned |
| 814 | Minimum Swaps To Make Sequences Increasing | medium | planned |
| 815 | Count Vowels Permutation | medium | planned |
| 816 | Paint House II | medium | planned |
| 817 | Best Time to Buy and Sell Stock III | hard | planned |
| 818 | Best Time to Buy and Sell Stock IV | hard | planned |
| 819 | Student Attendance Record II | hard | planned |
| 820 | Paint House III | hard | planned |
| 821 | Number of Music Playlists | hard | planned |

### Strings: Parsing & Manipulation (34)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 80 | Longest Common Prefix | easy | done |
| 81 | String to Integer | medium | done |
| 822 | Roman to Integer | easy | planned |
| 823 | Length of Last Word | easy | planned |
| 824 | Reverse Each Word | easy | planned |
| 825 | Detect Capital Usage | easy | planned |
| 826 | Add Binary | easy | planned |
| 827 | Add Strings | easy | planned |
| 828 | Goat Latin | easy | planned |
| 829 | License Key Formatting | easy | planned |
| 830 | Reformat Date | easy | planned |
| 831 | Shortest Distance to a Character | easy | planned |
| 832 | Decode the Message | easy | planned |
| 833 | Rearrange Spaces Between Words | easy | planned |
| 834 | Integer to Roman | medium | planned |
| 835 | Reverse Words in a String | medium | planned |
| 836 | Zigzag Conversion | medium | planned |
| 837 | Compare Version Numbers | medium | planned |
| 838 | Count and Say | medium | planned |
| 839 | Multiply Strings | medium | planned |
| 840 | Validate IP Address | medium | planned |
| 841 | Longest Absolute File Path | medium | planned |
| 842 | Fraction to Recurring Decimal | medium | planned |
| 843 | Reorder Log Files | medium | planned |
| 844 | Subdomain Visit Count | medium | planned |
| 845 | Mask Personal Information | medium | planned |
| 846 | Integer to English Words | medium | planned |
| 847 | One Edit Distance | medium | planned |
| 848 | Find And Replace in String | medium | planned |
| 849 | Expressive Words | medium | planned |
| 850 | Minimum Number of Swaps to Make the String Balanced | medium | planned |
| 851 | Valid Number | hard | planned |
| 852 | Text Justification | hard | planned |
| 853 | Strong Password Checker | hard | planned |

### Strings: Pattern Matching & Hashing (18)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 854 | Find the First Occurrence of a Substring | easy | planned |
| 855 | Rotate String | easy | planned |
| 856 | Repeated Substring Pattern | easy | planned |
| 857 | Maximum Repeating Substring | easy | planned |
| 858 | Words That Are Substrings of Others | easy | planned |
| 859 | Repeated String Match | medium | planned |
| 860 | Repeated DNA Sequences | medium | planned |
| 861 | Find All Pattern Occurrences | medium | planned |
| 862 | Prefix Function of a String | medium | planned |
| 863 | Longest Happy Prefix | medium | planned |
| 864 | Find Beautiful Indices | medium | planned |
| 865 | Minimum Time to Revert Word | medium | planned |
| 866 | Find Substring With Given Hash Value | medium | planned |
| 867 | Count Distinct Substrings | medium | planned |
| 868 | Distinct Echo Substrings | medium | planned |
| 869 | Longest Duplicate Substring | hard | planned |
| 870 | Sum of Prefix Match Scores | hard | planned |
| 871 | Shortest Palindrome | hard | planned |

### Strings: Palindromes (17)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 59 | Longest Palindromic Substring | medium | done |
| 82 | Valid Palindrome | easy | done |
| 83 | Palindromic Substrings | medium | done |
| 872 | Palindrome Number | easy | planned |
| 873 | Valid Palindrome With One Deletion | easy | planned |
| 874 | Lexicographically Smallest Palindrome | easy | planned |
| 875 | Palindrome Permutation | easy | planned |
| 876 | Break a Palindrome | medium | planned |
| 877 | Construct K Palindrome Strings | medium | planned |
| 878 | Longest Palindrome From Two-Letter Words | medium | planned |
| 879 | Palindrome Queries on Substrings | medium | planned |
| 880 | Largest Palindromic Number | medium | planned |
| 881 | Split Two Strings to Make a Palindrome | medium | planned |
| 882 | Palindrome Pairs | hard | planned |
| 883 | Find the Closest Palindrome | hard | planned |
| 884 | Super Palindromes | hard | planned |
| 885 | Max Product of Two Odd Palindromes | hard | planned |

### Math & Number Theory (25)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 886 | Fizz Buzz | easy | planned |
| 887 | Plus One | easy | planned |
| 888 | Happy Number | easy | planned |
| 889 | Power of Three | easy | planned |
| 890 | Ugly Number | easy | planned |
| 891 | Add Digits | easy | planned |
| 892 | Excel Sheet Column Number | easy | planned |
| 893 | Excel Sheet Column Title | easy | planned |
| 894 | Perfect Number | easy | planned |
| 895 | Self Dividing Numbers | easy | planned |
| 896 | Greatest Common Divisor of Strings | easy | planned |
| 897 | Find Greatest Common Divisor of Array | easy | planned |
| 898 | Count Odd Numbers in an Interval Range | easy | planned |
| 899 | Reverse Integer | medium | planned |
| 900 | Pow(x, n) | medium | planned |
| 901 | Factorial Trailing Zeroes | medium | planned |
| 902 | Count Primes | medium | planned |
| 903 | Ugly Number II | medium | planned |
| 904 | Fraction Addition and Subtraction | medium | planned |
| 905 | Nth Digit | medium | planned |
| 906 | Super Pow | medium | planned |
| 907 | The kth Factor of n | medium | planned |
| 908 | Consecutive Numbers Sum | medium | planned |
| 909 | Closest Prime Numbers in Range | medium | planned |
| 910 | Smallest Good Base | hard | planned |

### Combinatorics (11)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 911 | Pascal's Triangle | easy | planned |
| 912 | Binomial Coefficients Modulo a Prime | medium | planned |
| 913 | Permutation Sequence | medium | planned |
| 914 | Count Anagrams | medium | planned |
| 915 | Poor Pigs | medium | planned |
| 916 | Vowels of All Substrings | medium | planned |
| 917 | Number of Ways to Reach a Position After Exactly k Steps | medium | planned |
| 918 | Count All Valid Pickup and Delivery Options | hard | planned |
| 919 | Sum of Subsequence Widths | hard | planned |
| 920 | Count Ways to Make Array With Product | hard | planned |
| 921 | Count the Number of Ideal Arrays | hard | planned |

### Bit Manipulation (29)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 108 | Single Number | easy | done |
| 109 | Counting Bits | easy | done |
| 110 | Missing Number | easy | done |
| 922 | Number of 1 Bits | easy | planned |
| 923 | Reverse Bits | easy | planned |
| 924 | Power of Two | easy | planned |
| 925 | Power of Four | easy | planned |
| 926 | Hamming Distance | easy | planned |
| 927 | Number Complement | easy | planned |
| 928 | Binary Number with Alternating Bits | easy | planned |
| 929 | Sort Integers by the Number of 1 Bits | easy | planned |
| 930 | Decode XORed Array | easy | planned |
| 931 | Find the Original Array of Prefix Xor | easy | planned |
| 932 | Prime Number of Set Bits in Binary Representation | easy | planned |
| 933 | Total Hamming Distance | medium | planned |
| 934 | Single Number II | medium | planned |
| 935 | Single Number III | medium | planned |
| 936 | Bitwise AND of Numbers Range | medium | planned |
| 937 | Sum of Two Integers | medium | planned |
| 938 | Divide Two Integers | medium | planned |
| 939 | UTF-8 Validation | medium | planned |
| 940 | Maximum Product of Word Lengths | medium | planned |
| 941 | Minimum Flips to Make a OR b Equal to c | medium | planned |
| 942 | Decode XORed Permutation | medium | planned |
| 943 | Count Triplets That Can Form Two Arrays of Equal XOR | medium | planned |
| 944 | Gray Code | medium | planned |
| 945 | Smallest Subarrays With Maximum Bitwise OR | medium | planned |
| 946 | Minimum One Bit Operations to Make Integers Zero | hard | planned |
| 947 | Find XOR Sum of All Pairs Bitwise AND | hard | planned |

### Geometry (14)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 948 | Rectangle Overlap | easy | planned |
| 949 | Check If It Is a Straight Line | easy | planned |
| 950 | Valid Boomerang | easy | planned |
| 951 | Minimum Time Visiting All Points | easy | planned |
| 952 | Largest Triangle Area | easy | planned |
| 953 | Find Nearest Point That Has the Same X or Y Coordinate | easy | planned |
| 954 | Surface Area of 3D Shapes | easy | planned |
| 955 | Rectangle Area | medium | planned |
| 956 | Valid Square | medium | planned |
| 957 | Minimum Area Rectangle | medium | planned |
| 958 | Queries on Number of Points Inside a Circle | medium | planned |
| 959 | Max Points on a Line | hard | planned |
| 960 | Erect the Fence | hard | planned |
| 961 | Maximum Number of Darts Inside of a Circular Dartboard | hard | planned |

### Matrix & Simulation (19)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 104 | Rotate Image | medium | done |
| 105 | Spiral Matrix | medium | done |
| 962 | Toeplitz Matrix | easy | planned |
| 963 | Reshape the Matrix | easy | planned |
| 964 | Lucky Numbers in a Matrix | easy | planned |
| 965 | Flipping an Image | easy | planned |
| 966 | Determine Whether a Matrix Can Be Obtained by Rotation | easy | planned |
| 967 | Cells with Odd Values in a Matrix | easy | planned |
| 968 | Matrix Diagonal Sum | easy | planned |
| 969 | Transpose Matrix | easy | planned |
| 970 | Set Matrix Zeroes | medium | planned |
| 971 | Spiral Matrix II | medium | planned |
| 972 | Game of Life | medium | planned |
| 973 | Diagonal Traverse | medium | planned |
| 974 | Robot Bounded in Circle | medium | planned |
| 975 | Candy Crush | medium | planned |
| 976 | Spiral Matrix III | medium | planned |
| 977 | Walking Robot Simulation | medium | planned |
| 978 | Rotating the Box | medium | planned |

### Range Queries (Segment/Fenwick/Sparse Table) (12)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 979 | Range Sum Query - Mutable | medium | planned |
| 980 | Range Minimum Queries | medium | planned |
| 981 | Range Frequency Queries | medium | planned |
| 982 | Queries on a Permutation With Key | medium | planned |
| 983 | Range Sum Query 2D - Mutable | medium | planned |
| 984 | Count of Smaller Numbers After Self | hard | planned |
| 985 | Reverse Pairs | hard | planned |
| 986 | Count of Range Sum | hard | planned |
| 987 | Falling Squares | hard | planned |
| 988 | Create Sorted Array through Instructions | hard | planned |
| 989 | Longest Increasing Subsequence II | hard | planned |
| 990 | Count Good Triplets in an Array | hard | planned |

### Advanced Techniques (10)

| ID | Title | Difficulty | Status |
|---:|---|---|---|
| 991 | Maximum Population Year | easy | planned |
| 992 | Jump Game VI | medium | planned |
| 993 | Number of Flowers in Full Bloom | medium | planned |
| 994 | Maximum Number of Visible Points | medium | planned |
| 995 | Perfect Rectangle | medium | planned |
| 996 | Minimum Operations to Make a Subsequence | medium | planned |
| 997 | Constrained Subsequence Sum | hard | planned |
| 998 | Closest Subsequence Sum | hard | planned |
| 999 | Partition Array Into Two Arrays to Minimize Sum Difference | hard | planned |
| 1000 | Rectangle Area II | hard | planned |
