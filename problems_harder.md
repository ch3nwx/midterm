# Midterm 1 Practice Problems — Hard Edition

---

## Problem 1 — GCD+

You are given $T$ test cases. In each test case, you are given $K$ positive integers. Compute both the **GCD** (greatest common divisor) and the **LCM** (least common multiple) of all $K$ numbers.

### Input

- Line 1: integer $T$ ($1 \leq T \leq 100000$) — number of test cases.
- Each test case occupies one line: first an integer $K$ ($2 \leq K \leq 20$), followed by $K$ positive integers, each up to $10^{18}$.

### Output

For each test case, output two space-separated integers on one line: the GCD and the LCM of all $K$ numbers. If the LCM exceeds $10^{18}$, output `overflow` instead of the LCM value.

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

Hodilo restaurant's queuing system needs an upgrade. The rules are the same as before, but now guests who cannot be immediately seated are placed in a **FIFO waitlist**. Each time a guest leaves (freeing a table), the system immediately checks the front of the waitlist and seats them if a suitable table is now available.

Additionally, guests may cancel their wait. A cancellation removes that guest from the waitlist entirely (they never get seated). A guest who is already seated cannot cancel.

You must implement the following functions in `function.h`:

- `Table* createTable()` — same as before.
- `Guest* createGuest()` — now also reads a possible `cancel_time` per guest (the minute at which they cancel their wait, or `-1` if they will not cancel).
- `Guest* checkLeave(Table* tables, int tableCount, int currentTime)` — same semantics as before.
- `int assignTable(Table* tables, int tableCount, Guest* guest)` — same semantics as before.
- `void processWaitlist(Table* tables, int tableCount, Queue* waitlist, int currentTime)` — attempt to seat the front-of-queue guest; repeat as long as a guest at the front can be seated. Remove any guests from the front of the queue who have already cancelled.

### Input

- Line 1: integer $N$ ($1 \leq N \leq 50$) — number of tables.
- Line 2: $N$ integers $x_i$ ($1 \leq x_i \leq 4$) — table capacities.
- Line 3: integer $M$ ($1 \leq M \leq 200$) — number of incoming guests.
- Next $M$ lines: `guest_name` (at most 10 chars), `group_size` (1–4), `arrival_time` (660–900), `dining_time` (1–300), `cancel_time` (660–900 or `-1`). Arrival times are in non-decreasing order.

### Output

Same format as the original problem, with these additions:
- When a guest is seated from the waitlist: `hh:mm (minutes) -> guestName: enter (from waitlist)`
- When a guest cancels: `hh:mm (minutes) -> guestName: cancel`

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

You are given $T$ neural link nodes. Each node is now represented as a **64-bit unsigned integer**. Apply the following four rules **in sequence** to each node and output the result.

**Rule 1 — Overload Detection:**
If any two consecutive bits are both 1 (i.e., `S & (S >> 1) != 0`), the node is overloaded. Output `0xFFFFFFFFFFFFFFFF` and skip the remaining rules.

**Rule 2 — Neural Bridging:**
Find the position of the highest set bit (MSB) and the lowest set bit (LSB) in $S$. Fill all bits between them (inclusive) with 1s. If $S = 0$, output `0x0000000000000000` and skip Rules 3 and 4.

**Rule 3 — Checksum Integration:**
XOR all eight bytes of `bridge` together to produce a single checksum byte. Replace the lowest byte of `bridge` with this checksum.

**Rule 4 — Rotation:**
Count the number of set bits (popcount) in the result from Rule 3. Rotate the result **right** by that many bit positions (64-bit rotation).

### Input

- Line 1: integer $T$ — number of test cases.
- Next $T$ lines: each contains a 64-bit unsigned integer, given in **either** decimal or `0x`-prefixed hexadecimal format. You must handle both.

### Output

For each test case, output the result as a zero-padded 16-digit uppercase hexadecimal value with `0x` prefix (e.g., `0x0000000000000001`).

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

You are given an initial mysterious string and $q$ queries. The mysterious string format and the working string $S$ are defined the same way as in the original problem.

**Queries (four types):**

- `Insert idx str` — Decode `str` into a hex string and insert it into $S$ at position `idx`.
- `Remove idx len` — Delete `len` characters from $S$ starting at index `idx`.
- `Reverse idx1 idx2` — Reverse the substring of $S$ from index `idx1` to `idx2` (inclusive).
- `Replace old new` — Replace every non-overlapping occurrence of the hex substring `old` in $S$ with the hex substring `new`. Replacements are applied left-to-right. `old` and `new` are plain hexadecimal strings (not mysterious-string format).

After all queries, decode the final $S$ into ASCII: every consecutive pair of hex digits maps to one ASCII character.

### Input

- Line 1: the initial mysterious string in `d_1|d_2|...|d_m` format.
- Line 2: integer $q$ ($1 \leq q \leq 1000$).
- Next $q$ lines: query descriptions as specified above.

### Constraints

- The length of $S$ never exceeds 100,000 characters at any point.
- For `Replace`: `old` is guaranteed to be non-empty; `new` may be empty (deletion).

### Output

Output the final decoded ASCII string on a single line.

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

A billboard system stores text as a one-dimensional array of $N$ strings. Length tracking and lack of null terminators work the same as in the original problem. All strings are initially empty.

You must process $Q$ queries of the following types:

- `0 i c` — Append the single character `c` to the end of string $i$.
- `1 i s` — Append the string `s` to the end of string $i$.
- `2` — Print all non-empty strings in order from index $0$ to $N-1$, one per line.
- `3 i j` — Swap strings $i$ and $j$ (including their lengths).
- `4 i k` — Delete the first $k$ characters of string $i$. If $k \geq$ `len[i]`, the string becomes empty.
- `5 i j s` — Insert the string `s` into string $i$ at position $j$ (0-indexed, inserting before the character currently at position $j$). If $j =$ `len[i]`, this is equivalent to appending.

### Input

- Line 1: two integers $N$ ($1 \leq N \leq 1000$) and $Q$ ($1 \leq Q \leq 10000$).
- Next $Q$ lines: queries as described above.

### Output

For every type-`2` query, print all non-empty strings in index order, each on its own line.

### Constraints

- Total characters across all strings never exceed $10^6$ at any time.
- All indices are valid (in-bounds) unless stated otherwise.

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

There are $N \times M$ spires arranged in a 2D grid. Each spire at position $(r, c)$ has height $H[r][c]$ and broadcasts energy $V[r][c]$ to its four immediate neighbours (up, down, left, right). A neighbour absorbs the energy only if it is **strictly taller** than the broadcasting spire. Energy from multiple neighbours accumulates.

You are given $Q$ queries. Each query specifies a cell $(r, c)$ whose height is **toggled**: if it was previously modified, it reverts to its original height; otherwise, it changes to a new given height $H'$. After each toggle, output the 0-indexed cell (in row-major order, i.e., index $= r \times M + c$) that receives the maximum total absorbed energy, along with the amount. If multiple cells are tied, output the one with the smaller row-major index.

### Input

- Line 1: three integers $N$, $M$ ($1 \leq N, M \leq 1000$) and $Q$ ($1 \leq Q \leq 10000$).
- Next $N$ lines: each contains $M$ pairs of integers $H[r][c]$ ($|H[r][c]| \leq 2 \times 10^9$) and $V[r][c]$ ($0 \leq V[r][c] \leq 10^9$), space-separated.
- Next $Q$ lines: each contains three integers $r$, $c$, $H'$ — toggle the height of cell $(r, c)$ to $H'$ (or revert it if it was already toggled).

### Output

For each query, output two space-separated values: the row-major index of the cell receiving maximum energy, and the total energy it receives.

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

Fujiwara no Sai is now analyzing more complex chess puzzles. He wants to know all ways to capture the opponent's King in **3 moves or fewer**.

The chessboard, piece types, and movement rules are the same as in the original problem. Additionally, a Pawn that reaches the last row is **immediately promoted** to a Queen before the current move sequence continues.

Find the **minimum** number of moves needed to capture the opponent's King. Output **all distinct move sequences** of that minimum length, sorted in **lexicographic order** by the sequence of moves (each move written as `startPos endPos`, compared character by character).

If the King cannot be captured in 3 moves or fewer, output `0`.

### Input

- Line 1: integer $t$ — number of test cases.
- For each test case:
  - Line 1: integer $p$ — number of Sai's pieces.
  - Next $p$ lines: piece type and position.
  - Line: integer $o$ — number of opponent's pieces.
  - Next $o$ lines: opponent piece type and position.

### Output

For each test case:
- Line 1: the minimum number of moves $n$ (or `0` if impossible).
- If $n > 0$: output all distinct shortest move sequences, one sequence per block. Each sequence block contains $n$ lines (one move per line as `startPos endPos`). Separate sequences with a blank line. Sort sequences lexicographically.

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

$n$ mages are undergoing the First-Class Mage Exam. Each mage has a name and a mana value ($-99$ to $99$). You are given $q$ queries; each query either sorts or modifies the roster.

**Sorting Rules (rules 0–7):**

| Rule | Sort key | Tiebreaker |
|------|----------|------------|
| 0 | `value` ascending | Name lexicographically ascending |
| 1 | `value²` ascending | Name lexicographically ascending |
| 2 | Digit sum of `|value|` ascending | Name lexicographically ascending |
| 3 | Swap tens and units digits of `value`, preserve sign, sort ascending | Name lexicographically ascending |
| 4 | Prime `|value|` first (ascending by original value), non-prime after (ascending by original value) | Name lexicographically ascending |
| 5 | Number of distinct prime factors of `|value|` ascending (0 and 1 have 0 prime factors) | Name lexicographically ascending |
| 6 | Sum of all positive divisors of `|value|` ascending (0 has divisor sum 0; 1 has divisor sum 1) | Name lexicographically ascending |
| 7 | Length of the longest non-decreasing subsequence of the binary representation of `|value|` **descending** | Name lexicographically ascending |

**Query types:**

- `S rule` — Sort using the given rule (0–7) and print all names, one per line.
- `A name value` — Add a new mage with the given name and value to the roster.
- `D name` — Remove the mage with the given name from the roster.

### Input

- Line 1: two integers $n$ and $q$ ($1 \leq n \leq 1000$, $1 \leq q \leq 1000$).
- Next $n$ lines: `name` (at most 30 characters) and `value` ($-99 \leq \text{value} \leq 99$).
- Next $q$ lines: queries in the format described above.

### Output

For each `S` query, output the sorted names one per line, followed by a blank line.

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
