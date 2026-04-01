# Midterm 1 Practice Problems — Hard Edition

---

## Problem 1 — GCD+

You are given $T$ test cases. In each test case you receive $K$ positive integers. Compute both the **GCD** (greatest common divisor) and the **LCM** (least common multiple) of all $K$ numbers.

### Input

- Line 1: integer $T$ ($1 \leq T \leq 100\,000$) — number of test cases.
- Each test case: one line starting with $K$ ($2 \leq K \leq 20$), followed by $K$ positive integers each up to $10^{18}$.

### Output

For each test case, output two space-separated values: the GCD and the LCM. If the LCM exceeds $10^{18}$, output `overflow` in place of the LCM.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 20 | $T \leq 100$, $K = 2$, all values $\leq 10^6$ |
| 2 | 20 | $T \leq 1000$, $K = 2$, values $\leq 10^{18}$; LCM never overflows |
| 3 | 20 | $T \leq 10^4$, $K \leq 10$, values $\leq 10^{18}$; LCM may overflow |
| 4 | 20 | $T \leq 10^5$, $K \leq 20$, full constraints |
| 5 | 20 | Same as subtask 4 but time-tight; requires $O(K \log V)$ per test case |

### Sample Input

```
3
2 2 4
3 6 9 12
4 12 15 20 30
```

### Sample Output

```
2 4
3 36
1 60
```

**Time limit:** 2 sec | **Memory limit:** 64 MB

---

## Problem 2 — Table Management System ver.3

Same restaurant simulation as before, but now guests who cannot be immediately seated are placed in a **FIFO waitlist**. Each time a table frees up, the system immediately and repeatedly tries to seat the front of the waitlist until no more guests can be placed. Guests may also **cancel** their wait before being seated.

Simulation runs from minute 660 to minute 900. At each minute:

1. Check for departing guests — free their tables, then immediately attempt to seat waitlisted guests (front-first, repeatedly until no suitable table remains or the list is empty).
2. Check if the next guest arrives. Try to seat them immediately; if no suitable table exists, add them to the back of the waitlist.
3. Process any cancellations scheduled for this minute — remove those guests from the waitlist (they are never seated).
4. Print table snapshot at minutes 660, 720, 780, 840, and 900.

### Input

- Line 1: $N$ ($1 \leq N \leq 50$) — tables.
- Line 2: $N$ capacities $x_i$ ($1 \leq x_i \leq 4$).
- Line 3: $M$ ($1 \leq M \leq 200$) — guests.
- Next $M$ lines: `name` (≤ 10 chars), `group_size` (1–4), `arrival_time` (660–900), `dining_time` (1–300), `cancel_time` (660–900 or `-1` if they will not cancel). Arrival times are non-decreasing.

### Output

Same format as the original problem, with additions:
- Seated from waitlist: `hh:mm (min) -> name: enter (from waitlist)`
- Guest cancels: `hh:mm (min) -> name: cancel`

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 20 | $N, M \leq 10$; no cancellations; waitlist always has at most 1 guest |
| 2 | 20 | $N, M \leq 30$; no cancellations; multiple waitlisted guests |
| 3 | 20 | $N, M \leq 50$; cancellations present; cancelled guest is always at the front of the list |
| 4 | 20 | $N \leq 50$, $M \leq 100$; cancellations may target any position in the list |
| 5 | 20 | $N \leq 50$, $M \leq 200$; full constraints |

### Sample Input

```
2
2 2
4
Alice 2 700 120 -1
Bob 2 710 60 -1
Carol 2 720 30 780
Dave 2 730 60 -1
```

### Sample Output

```
11:00 (660) -> table: |||
11:40 (700) -> Alice: enter
11:50 (710) -> Bob: enter
12:00 (720) -> table: |Alice(100min)|Bob(50min)|
13:00 (780) -> Bob: leave
13:00 (780) -> Carol: cancel
13:00 (780) -> Dave: enter (from waitlist)
13:00 (780) -> table: |Alice(40min)|Dave(50min)|
14:00 (840) -> Alice: leave
14:00 (840) -> Dave: leave
15:00 (900) -> table: |||
```

**Time limit:** 1 sec | **Memory limit:** 512 MB

---

## Problem 3 — Neural Link Node Repair II

Same as the original, but each node is a **64-bit unsigned integer** with **four** rules.

**Rule 1 — Overload Detection:**
If `S & (S >> 1) != 0`, output `0xFFFFFFFFFFFFFFFF` and skip the rest.

**Rule 2 — Neural Bridging:**
Fill all bits between MSB and LSB (inclusive) with 1s. If $S = 0$, output `0x0000000000000000` and skip Rules 3 and 4.

**Rule 3 — Checksum Integration:**
XOR all **eight** bytes of `bridge` into a checksum byte. Replace the lowest byte of `bridge` with this checksum.

**Rule 4 — Rotation:**
Count the set bits (popcount) of the Rule 3 result. Rotate the result **right** by that many positions (64-bit rotation).

### Input

- Line 1: integer $T$ — test cases.
- Next $T$ lines: a 64-bit unsigned integer in either **decimal** or **`0x`-prefixed hexadecimal**.

### Output

Zero-padded 16-digit uppercase hex with `0x` prefix for each test case.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | All inputs trigger Rule 1 or are zero; only decimal input format |
| 2 | 15 | No Rule 1 triggers; single set bit (MSB = LSB); only hex input |
| 3 | 20 | Mix of all rules; only hex input; $T \leq 1000$ |
| 4 | 25 | Mix of all rules; both decimal and hex input; $T \leq 10000$ |
| 5 | 25 | Full constraints; $T \leq 100000$ |

### Sample Input

```
4
0x0000000000000003
0x0000000000000000
512
0x0800000000000002
```

### Sample Output

```
0xFFFFFFFFFFFFFFFF
0x0000000000000000
0x0000000000000200
0x01FFFFFFFFFFFFFF
```

**Time limit:** 1 sec | **Memory limit:** 32 MB

---

## Problem 4 — It Is Not Simple

Same mysterious string format and working string $S$, but with **four** query types.

**Queries:**
- `Insert idx str` — decode `str` and insert into $S$ at position `idx`.
- `Remove idx len` — delete `len` characters from $S$ starting at `idx`.
- `Reverse idx1 idx2` — reverse the substring of $S$ from `idx1` to `idx2` (inclusive).
- `Replace old new` — replace every non-overlapping occurrence of hex substring `old` in $S$ with `new` (left-to-right). `old` and `new` are plain hex strings (not mysterious format). `new` may be empty (deletion).

After all queries, decode the final $S$ to ASCII as before.

### Input

- Line 1: initial mysterious string.
- Line 2: $q$ ($1 \leq q \leq 1000$).
- Next $q$ lines: queries.

### Constraints

$|S| \leq 100\,000$ at all times.

### Output

The final decoded ASCII string on a single line.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | $q \leq 20$; only `Insert` and `Remove`; $|S| \leq 1000$ |
| 2 | 20 | $q \leq 100$; adds `Reverse`; $|S| \leq 5000$ |
| 3 | 20 | $q \leq 500$; adds `Replace`; `old` always has length 2 (one byte); $|S| \leq 10000$ |
| 4 | 20 | $q \leq 1000$; `Replace` with arbitrary-length `old`; $|S| \leq 50000$ |
| 5 | 25 | Full constraints; $|S| \leq 100000$; time-tight |

### Sample Input

```
20065|16019311|33981975|40140414
4
Insert 6 13183871|30367|126|30359|204916
Remove 3 12
Replace 7E 7E7E
Insert 7 3616|1175|1056
```

### Sample Output

```
No~~~~ It is too hard~~~~
```

**Time limit:** 2 sec | **Memory limit:** 64 MB

---

## Problem 5 — NEWater II

Same billboard string array as the original. No null terminators; `len[i]` is authoritative. All strings initially empty.

Process $Q$ queries:

- `0 i c` — append character `c` to string $i$.
- `1 i s` — append string `s` to string $i$.
- `2` — print all non-empty strings in index order, one per line.
- `3 i j` — swap strings $i$ and $j$ (including their lengths).
- `4 i k` — delete the first $k$ characters of string $i$. If $k \geq \text{len}[i]$, the string becomes empty.
- `5 i j s` — insert string `s` into string $i$ at position $j$ (0-indexed, before the character currently at $j$). If $j = \text{len}[i]$, equivalent to appending.

### Input

- Line 1: $N$ ($1 \leq N \leq 1000$) and $Q$ ($1 \leq Q \leq 10\,000$).
- Next $Q$ lines: queries.

### Constraints

Total characters across all strings $\leq 10^6$ at all times. All indices are valid.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | $N \leq 10$, $Q \leq 100$; only types 0–2 (same as original) |
| 2 | 20 | $N \leq 50$, $Q \leq 500$; adds type-3 (swap) |
| 3 | 20 | $N \leq 100$, $Q \leq 1000$; adds type-4 (delete prefix) |
| 4 | 20 | $N \leq 500$, $Q \leq 5000$; adds type-5 (insert at position) |
| 5 | 25 | Full constraints |

### Sample Input

```
3 9
1 2 6KKPba5qeO
0 2 K
1 1 aqnoQc
0 1 6
0 0 a
3 0 2
4 1 3
5 0 1 XY
2
```

### Sample Output

```
aXY
noQc6
a6KKPba5qeOK
```

**Time limit:** 2 sec | **Memory limit:** 128 MB

---

## Problem 6 — Crystalline Spire Energy Harvest II

There is an $N \times M$ 2D grid of spires. Each cell $(r, c)$ broadcasts energy $V[r][c]$ to its four immediate neighbours (up, down, left, right). A neighbour absorbs only if it is **strictly taller**. Energy accumulates from multiple sources.

You receive $Q$ queries. Each query gives a cell $(r, c)$ and a new height $H'$. The height is **toggled**: if previously modified, it reverts to its original value; otherwise it changes to $H'$. After each toggle, output the cell (in row-major order: index $= r \times M + c$) that receives the maximum total absorbed energy, and the amount. On a tie, output the smaller row-major index.

### Input

- Line 1: $N$, $M$ ($1 \leq N, M \leq 1000$) and $Q$ ($1 \leq Q \leq 10\,000$).
- Next $N$ lines: each contains $M$ pairs $H[r][c]$ ($|H| \leq 2 \times 10^9$) and $V[r][c]$ ($0 \leq V \leq 10^9$), space-separated.
- Next $Q$ lines: $r$, $c$, $H'$ — toggle the height of cell $(r, c)$.

### Output

For each query, output two space-separated values: the row-major index of the max-energy cell and the total energy it receives.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | $N = 1$ (1D case); $M \leq 1000$; $Q \leq 100$ |
| 2 | 20 | $N, M \leq 50$; $Q \leq 100$; re-compute from scratch each query is acceptable |
| 3 | 20 | $N, M \leq 200$; $Q \leq 1000$ |
| 4 | 25 | $N, M \leq 500$; $Q \leq 5000$ |
| 5 | 20 | Full constraints; $N, M \leq 1000$; $Q \leq 10000$; time-tight |

### Sample Input

```
2 2 2
4 2 3 5
6 7 8 4
0 1 10
0 1 10
```

### Sample Output

```
2 9
3 7
```

**Time limit:** 3 sec | **Memory limit:** 256 MB

---

## Problem 7 — Check Checkmate II

Same chess setup as the original. Now find if the King can be captured in **3 moves or fewer**. A Pawn reaching the last row is **immediately promoted to a Queen** before the sequence continues.

Find the **minimum** number of moves needed. Output **all distinct move sequences** of that minimum length, sorted **lexicographically** (each move written as `start end`, sequences compared character by character). If the King cannot be captured in 3 moves or fewer, output `0`.

### Input

Same format as the original.

### Output

- Line 1: minimum number of moves $n$ (or `0`).
- If $n > 0$: all distinct shortest sequences, one per block of $n$ lines each. Separate sequences with a blank line. Sort sequences lexicographically.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | $t \leq 3$; King capturable in 1 move; no promotion |
| 2 | 20 | $t \leq 5$; 1 or 2 moves; only one valid sequence exists; no promotion |
| 3 | 20 | $t \leq 5$; 1–2 moves; multiple valid sequences; no promotion |
| 4 | 25 | $t \leq 10$; 1–3 moves; multiple sequences; no promotion |
| 5 | 20 | $t \leq 10$; full constraints including pawn promotion |

### Sample Input

```
1
2
Q G6
K G7
1
K H8
```

### Sample Output

```
2
G6 H6
H6 H8
```

**Time limit:** 3 sec | **Memory limit:** 64 MB

---

## Problem 8 — Magical Evaluation II

Same mage exam as the original, but with **8 sorting rules** and **dynamic add/remove** queries.

**Sorting Rules (0–7):**

| Rule | Sort key | Tiebreaker |
|------|----------|------------|
| 0 | `value` ascending | Name lex. ascending |
| 1 | `value²` ascending | Name lex. ascending |
| 2 | Digit sum of `|value|` ascending | Name lex. ascending |
| 3 | Swap tens+units digits, preserve sign, ascending | Name lex. ascending |
| 4 | Prime `|value|` first (asc), non-prime after (asc) | Name lex. ascending |
| 5 | Number of **distinct** prime factors of `|value|` ascending (0 and 1 → 0 factors) | Name lex. ascending |
| 6 | Sum of all positive divisors of `|value|` ascending (0 → sum 0; 1 → sum 1) | Name lex. ascending |
| 7 | Length of longest non-decreasing subsequence of binary representation of `|value|`, **descending** | Name lex. ascending |

**Query types:**

- `S rule` — sort by the given rule (0–7) and print all names, one per line, followed by a blank line.
- `A name value` — add a mage with the given name and value.
- `D name` — remove the mage with the given name.

### Input

- Line 1: $n$ ($1 \leq n \leq 1000$) and $q$ ($1 \leq q \leq 1000$).
- Next $n$ lines: `name` (≤ 30 chars) and `value` (−99 to 99).
- Next $q$ lines: queries.

### Output

For each `S` query, output sorted names one per line, followed by a blank line.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | No `A`/`D` queries; only Rules 0–4 (same as original) |
| 2 | 15 | No `A`/`D` queries; Rules 0–6 |
| 3 | 20 | No `A`/`D` queries; all Rules 0–7 |
| 4 | 25 | `A`/`D` queries present; Rules 0–6; roster never empty |
| 5 | 25 | Full constraints; all Rules 0–7; `A`/`D` queries; roster may become empty |

### Sample Input

```
3 5
Frieren 7
Fern -7
Denken 1
S 0
A Ubel 0
S 4
D Frieren
S 1
```

### Sample Output

```
Fern
Denken
Frieren

Ubel
Denken
Fern
Frieren

Ubel
Denken
Fern

```

**Time limit:** 2 sec | **Memory limit:** 64 MB
