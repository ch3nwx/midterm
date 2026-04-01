# Midterm 1 Practice Problems

---

## Problem 1 — 9463: GCD

Given two positive integers $a$ and $b$, compute their greatest common divisor (GCD) — the largest integer that divides both with no remainder.

### Input

- Line 1: integer $t$ — number of test cases.
- Next $t$ lines: two positive integers $a$ and $b$.

### Output

For each test case, print the GCD of $a$ and $b$ on a single line.

### Subtasks

| Subtask | Points | $t \leq$ | $a, b \leq$ |
|---------|--------|-----------|-------------|
| 1 | 25 | 100 | 100 |
| 2 | 25 | 10 000 | 10 000 |
| 3 | 25 | 20 000 | 100 000 |
| 4 | 25 | 25 000 | 1 000 000 |

### Sample Input

```
3
2 4
6 9
10 15
```

### Sample Output

```
2
3
5
```

**Time limit:** 1 sec | **Memory limit:** 32 MB

---

## Problem 2 — 14151: Table Management System

Hodilo is a well-known hot pot restaurant, and it is extremely challenging to get a table during peak dining hours. The restaurant does not accept reservations in advance — guests visit, take a number, and wait. Write a program that simulates the entire queuing and seating system.

The simulation runs from **11:00 (minute 660)** to **15:00 (minute 900)** inclusive. At each minute:

1. Check if any seated guest's dining time has expired. If so, they leave and the table becomes empty. Process **all** departures before any arrivals at the same minute.
2. Check if the next guest in the input list arrives at this minute. Assign them to the **first** empty table whose capacity $\geq$ their group size. If no such table is available, the guest leaves without being seated. Continue assigning all guests arriving at this minute.
3. At minutes 660, 720, 780, 840, and 900, print the current table snapshot.

### Input

- Line 1: integer $N$ ($1 \leq N \leq 50$) — number of tables.
- Line 2: $N$ integers $x_i$ ($1 \leq x_i \leq 4$) — capacity of each table.
- Line 3: integer $M$ ($1 \leq M \leq 50$) — number of guests.
- Next $M$ lines: `name` (≤ 10 chars), `group_size` (1–4), `arrival_time` (660–900), `dining_time` (1–300). Arrival times are non-decreasing.

### Output

- Guest seated: `hh:mm (minutes) -> name: enter`
- Guest leaves: `hh:mm (minutes) -> name: leave`
- Table snapshot: `hh:mm (minutes) -> table: |name(Xmin)|...|name(Xmin)|` (empty table = `||`)

Events at the same minute: departures first, then arrivals, then snapshot (if on the hour).

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 20 | $N, M \leq 5$; all guests can always be seated immediately |
| 2 | 20 | $N, M \leq 10$; no two guests share the same arrival time |
| 3 | 20 | $N, M \leq 30$; guests may arrive at the same minute |
| 4 | 20 | $N, M \leq 50$; a guest may arrive when no table is available (they leave immediately) |
| 5 | 20 | Full constraints |

### Sample Input

```
3
4 2 3
5
Alice 2 700 100
Mike 4 750 100
Doraemon 3 790 100
Kelvin 2 860 40
Bob 3 820 10
```

### Sample Output

```
11:00 (660) -> table: ||||
11:40 (700) -> Alice: enter
12:00 (720) -> table: |Alice(80min)|||
13:00 (780) -> table: |Alice(20min)|||
13:20 (800) -> Alice: leave
13:20 (800) -> Mike: enter
13:20 (800) -> Doraemon: enter
14:00 (840) -> table: |Mike(60min)||Doraemon(60min)|
14:20 (860) -> Kelvin: enter
15:00 (900) -> Mike: leave
15:00 (900) -> Kelvin: leave
15:00 (900) -> Doraemon: leave
15:00 (900) -> Bob: enter
15:00 (900) -> table: |Bob(10min)|||
```

**Time limit:** 1 sec | **Memory limit:** 512 MB

---

## Problem 3 — 14900: Neural Link Node Repair

You are given $T$ neural link nodes. Each node is a 32-bit unsigned integer. Apply the following three rules **in sequence** to each node and output the result.

**Rule 1 — Overload Detection:**
If any two consecutive bits are both 1 (i.e., `S & (S >> 1) != 0`), the node is overloaded. Output `0xFFFFFFFF` and skip Rules 2 and 3.

**Rule 2 — Neural Bridging:**
Find the highest set bit (MSB) and lowest set bit (LSB). Fill all bits between them (inclusive) with 1s:
$$\text{bridge} = \bigl((1 \ll (\text{msb}+1)) - 1\bigr) \;\&\; \sim\bigl((1 \ll \text{lsb}) - 1\bigr)$$
If $S = 0$, output `0x00000000` and skip Rule 3.

**Rule 3 — Checksum Integration:**
XOR all four bytes of `bridge` together into a single checksum byte. Replace the lowest byte of `bridge` with this checksum.

### Input

- Line 1: integer $T$ — number of test cases.
- Next $T$ lines: a 32-bit unsigned integer with `0x` prefix.

### Output

For each test case, output the result as a zero-padded 8-digit uppercase hex value with `0x` prefix (e.g., `0x0A2B4C6D`).

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | $T \leq 10$; all inputs trigger Rule 1 or are zero |
| 2 | 15 | $T \leq 100$; MSB = LSB (single set bit), Rule 3 only XORs one non-zero byte |
| 3 | 20 | $T \leq 1000$; no input triggers Rule 1 |
| 4 | 25 | $T \leq 10000$; mix of all three rule paths |
| 5 | 25 | $T \leq 100000$; full constraints |

### Sample Input

```
4
0x00000003
0x00000000
0x00000008
0x08000002
```

### Sample Output

```
0xFFFFFFFF
0x00000000
0x00000008
0x0FFFFFF1
```

**Time limit:** 1 sec | **Memory limit:** 32 MB

---

## Problem 4 — 14901: It Is Very Simple

You are given an initial mysterious string and $q$ queries.

**Initialization:**
A mysterious string has format `d_1|d_2|...|d_m` where each $d_i$ is a decimal integer. To decode:
1. Convert each $d_i$ to its uppercase hexadecimal string with **no zero-padding** (e.g., `20065` → `4E61`).
2. Concatenate all hex strings to form the working string $S$.

**Queries (two types):**
- `Insert idx str` — decode `str` into hex string $S_\text{ins}$; insert it into $S$ at position `idx` (0-indexed, before the character currently at `idx`).
- `Remove idx len` — delete exactly `len` characters from $S$ starting at `idx`.

After all queries, decode the final $S$ to ASCII: every consecutive pair of hex digits maps to one ASCII character (e.g., `4E6F` → `No`).

### Input

- Line 1: the initial mysterious string.
- Line 2: integer $q$ ($1 \leq q \leq 100$).
- Next $q$ lines: queries as described.

### Constraints

$|S| \leq 10\,000$ at all times.

### Output

The final decoded ASCII string on a single line.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | $q \leq 5$; only `Insert` queries; no overlapping indices |
| 2 | 15 | $q \leq 20$; only `Remove` queries |
| 3 | 25 | $q \leq 50$; mix of both; each $d_i \leq 255$ (single hex byte) |
| 4 | 25 | $q \leq 100$; full constraints, $|S| \leq 5000$ |
| 5 | 20 | $q \leq 100$; $|S| \leq 10000$ |

### Sample Input

```
20065|16019311|33981975|40140414
3
Insert 6 13183871|30367|126|30359|204916
Remove 3 12
Insert 7 3616|1175|1056
```

### Sample Output

```
No~~ It is too hard~~
```

**Time limit:** 1 sec | **Memory limit:** 32 MB

---

## Problem 5 — 14903: NEWater

A billboard system stores text as an array of $N$ strings. Each string $i$ has a length value `len[i]`. There are **no null terminators** — `len[i]` is the sole indicator of how many characters are valid. All strings are initially empty.

Process $Q$ queries:

- `0 i c` — append character `c` to string $i$.
- `1 i s` — append string `s` to string $i$.
- `2` — print all non-empty strings in order (index 0 to $N-1$), one per line.

### Input

- Line 1: two integers $N$ ($1 \leq N \leq 100$) and $Q$ ($1 \leq Q \leq 1000$).
- Next $Q$ lines: queries as described.

### Output

For every type-`2` query, print all non-empty strings in index order, each on its own line.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | $N=1$, $Q \leq 20$; only type-`0` and type-`2` queries |
| 2 | 15 | $N \leq 5$, $Q \leq 50$; type-`1` strings contain only lowercase letters |
| 3 | 20 | $N \leq 20$, $Q \leq 200$; at most one type-`2` query |
| 4 | 25 | $N \leq 50$, $Q \leq 500$; multiple type-`2` queries |
| 5 | 25 | Full constraints |

### Sample Input

```
3 6
1 2 6KKPba5qeO
0 2 K
1 1 aqnoQc
0 1 6
0 0 a
2
```

### Sample Output

```
a
aqnoQc6
6KKPba5qeOK
```

**Time limit:** 1 sec | **Memory limit:** 32 MB

---

## Problem 6 — 14904: Crystalline Spire Energy Harvest

There are $N$ spires in a line. Spire $i$ has height $H[i]$ and broadcasts energy $V[i]$ to its immediate neighbours (left: $i-1$, right: $i+1$). A neighbour **absorbs** the energy only if it is **strictly taller** than spire $i$. Energy accumulates from multiple sources.

Find the spire that receives the most total absorbed energy. On a tie, output the one with the smaller index (0-indexed).

### Input

- Line 1: integer $N$ ($1 \leq N \leq 5 \times 10^6$).
- Next $N$ lines: $H[i]$ ($|H[i]| \leq 2 \times 10^9$) and $V[i]$ ($0 \leq V[i] \leq 10^9$).

### Output

Two space-separated integers: the 0-indexed spire number and the total energy it receives.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | $N \leq 10$; all heights are distinct and positive |
| 2 | 15 | $N \leq 1000$; heights are distinct |
| 3 | 20 | $N \leq 10^5$; heights may be equal (ties in height → no absorption) |
| 4 | 25 | $N \leq 10^6$; negative heights allowed |
| 5 | 25 | $N \leq 5 \times 10^6$; full constraints, time-sensitive |

### Sample Input

```
4
4 2
3 5
6 7
8 4
```

### Sample Output

```
3 7
```

**Time limit:** 1 sec | **Memory limit:** 128 MB

---

## Problem 7 — 14905: Check Checkmate

Fujiwara no Sai is analyzing chess puzzles. Determine if the opponent's King can be captured in **2 moves or fewer**.

Standard 8×8 board, columns A–H, rows 1–8. Piece movement rules:

| Piece | Movement |
|-------|----------|
| Q (Queen) | Any number of vacant squares horizontally, vertically, or diagonally |
| R (Rook) | Any number of vacant squares horizontally or vertically |
| N (Knight) | L-shape: 2 squares in one direction, 1 square perpendicular |
| B (Bishop) | Any number of vacant squares diagonally |
| P (Pawn) | Forward one square; captures diagonally forward one square; may move two from starting row |
| K (King) | One square in any direction |

A piece captures an opponent by moving onto its square.

### Input

- Line 1: integer $t$ — number of test cases.
- For each test case:
  - Line 1: $p$ — number of Sai's pieces.
  - Next $p$ lines: piece type and position (e.g., `Q H1`).
  - Line: $o$ — number of opponent's pieces.
  - Next $o$ lines: opponent piece type and position.

### Output

For each test case:
- If the King can be captured in 1 move: print `1`, then the move on the next line (`start end`, e.g., `H1 H8`).
- In 2 moves: print `2`, then both moves on separate lines.
- Otherwise: print `0`.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 15 | $t \leq 3$; King capturable in exactly 1 move; only Queens and Rooks |
| 2 | 20 | $t \leq 5$; 1-move capture possible; all piece types |
| 3 | 20 | $t \leq 5$; capture requires exactly 2 moves; no blocking pieces |
| 4 | 25 | $t \leq 10$; 2-move capture; opponent has blocking pieces |
| 5 | 20 | $t \leq 20$; full constraints; may require 0 (impossible) |

### Sample Input

```
2
8
R A1
R B1
N C1
P D1
B E1
P F1
K G1
Q H1
1
K H8
2
Q G6
K G7
1
K H8
```

### Sample Output

```
1
H1 H8
2
G6 H6
H6 H8
```

**Time limit:** 1 sec | **Memory limit:** 32 MB

---

## Problem 8 — 14906: Magical Evaluation

$n$ mages are taking the First-Class Mage Exam. Each mage has a name and a mana value (−99 to 99). For each of $q$ queries, sort all mages by the specified rule and output their names.

**Sorting Rules:**

| Rule | Sort key | Tiebreaker |
|------|----------|------------|
| 0 | `value` ascending | Name lexicographically ascending |
| 1 | `value²` ascending | Name lexicographically ascending |
| 2 | Digit sum of `|value|` ascending (e.g., −85 → 8+5=13) | Name lexicographically ascending |
| 3 | Swap tens and units digits, preserve sign, ascending (e.g., 78→87, −6→−60, 10→1) | Name lexicographically ascending |
| 4 | Prime `|value|` first (ascending by original value), non-prime after (ascending) | Name lexicographically ascending |

### Input

- Line 1: $n$ and $q$ ($1 \leq n, q \leq 100$).
- Next $n$ lines: `name` (≤ 30 chars) and `value`.
- Next $q$ lines: a rule number (0–4).

### Output

For each query, output sorted names one per line, followed by a blank line.

### Subtasks

| Subtask | Points | Constraints |
|---------|--------|-------------|
| 1 | 20 | $n, q \leq 10$; only Rule 0 |
| 2 | 20 | $n, q \leq 30$; only Rules 0 and 1; no ties |
| 3 | 20 | $n, q \leq 50$; Rules 0–2; ties possible |
| 4 | 20 | $n, q \leq 100$; Rules 0–3 |
| 5 | 20 | $n, q \leq 100$; all Rules 0–4 |

### Sample Input

```
7 5
Frieren 7
Fern -7
Denken 1
Ubel 0
Land -85
Wirbel 7
Kanne 10
0
1
2
3
4
```

### Sample Output

```
Land
Fern
Ubel
Denken
Frieren
Wirbel
Kanne

Land
Ubel
Denken
Frieren
Fern
Wirbel
Kanne

Ubel
Denken
Frieren
Fern
Wirbel
Kanne
Land

Ubel
Denken
Kanne
Frieren
Fern
Wirbel
Land

Ubel
Denken
Fern
Frieren
Wirbel
Kanne
Land
```

**Time limit:** 1 sec | **Memory limit:** 32 MB
