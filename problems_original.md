# Midterm 1 Practice Problems

---

## Problem 1 — 9463: GCD

Given two positive integers $a$ and $b$, compute the greatest common divisor (GCD) of $a$ and $b$. The GCD of $a$ and $b$ is the biggest integer that can divide both $a$ and $b$ with no remainder.

### Input

The first line contains a positive integer $t$ ($t \leq 1000$), which indicates the number of test cases. In the next $t$ lines, each line contains two positive integers $a$ and $b$, both $\leq 10^6$.

**Subtask constraints:**
- Case 1: $t \leq 100$, $a, b \leq 100$
- Case 2: $t \leq 10000$, $a, b \leq 10000$
- Case 3: $t \leq 20000$, $a, b \leq 100000$
- Case 4: $t \leq 25000$, $a, b \leq 1000000$

### Output

For each test case, output the GCD of $a$ and $b$ on a single line.

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

## Problem 2 — 14151: Table Management System ver.2

Hodilo is a well-known hot pot restaurant, and it is extremely challenging to get a table during peak dining hours. The restaurant does not accept reservations in advance, requiring every guest to visit the restaurant, take a number, and wait for their turn. You have been invited to design a queuing system that helps assign a table to each guest.

This is a **partial judge** problem. You must implement four helper functions in `function.h`:

- `Table* createTable()` — Read the number of tables and their sizes from input, allocate memory, and return a pointer to the Table array.
- `Guest* createGuest()` — Read the next guest's information (name, group size, arrival time, dining time) and return a pointer to a new Guest struct.
- `Guest* checkLeave(Table* tables, int tableCount, int currentTime)` — Scan all tables and return a pointer to the first guest whose departure time equals `currentTime`. If no guest is leaving, return `NULL`. (When a guest leaves, free the associated memory and mark the table as empty.)
- `int assignTable(Table* tables, int tableCount, Guest* guest)` — Find the first empty table whose size is $\geq$ `guest->groupSize`. If found, seat the guest and return the table index (0-based); otherwise return `-1`.

The main loop simulates time from **11:00 (minute 660)** to **15:00 (minute 900)** inclusive, advancing one minute at a time.

### Input

- Line 1: integer $N$ ($1 \leq N \leq 50$) — number of tables.
- Line 2: $N$ integers $x_i$ ($1 \leq x_i \leq 4$) — capacity of each table.
- Line 3: integer $M$ ($1 \leq M \leq 50$) — number of incoming guests.
- Next $M$ lines: each contains `guest_name` (at most 10 characters), `group_size` (1–4), `arrival_time` (660–900), `dining_time` (1–300 minutes). Guest arrival times are in non-decreasing order.

### Output

Output is already handled by `main.c`:
- When a guest enters or leaves: `hh:mm (minutes) -> guestName: enter` or `hh:mm (minutes) -> guestName: leave`
- At each full hour (minute 720, 780, 840, 900): `hh:mm (minutes) -> table: |guestName(remainingMin)| ... |guestName(remainingMin)|` (empty tables shown as `||`)

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

You are given $T$ neural link nodes. Each node is represented as a 32-bit unsigned integer. Apply the following three rules **in sequence** to each node and output the result.

**Rule 1 — Overload Detection:**
If any two consecutive bits are both 1 (i.e., `S & (S >> 1) != 0`), the node is overloaded. Output `0xFFFFFFFF` and skip Rules 2 and 3 for this node.

**Rule 2 — Neural Bridging:**
Find the position of the highest set bit (MSB) and the lowest set bit (LSB) in $S$. Fill all bits between them (inclusive) with 1s. Formally, compute:
$$\text{bridge} = \left((1 \ll (\text{msb}+1)) - 1\right) \;\&\; \sim\left((1 \ll \text{lsb}) - 1\right)$$
If $S = 0$, output `0x00000000` and skip Rule 3.

**Rule 3 — Checksum Integration:**
XOR all four bytes of `bridge` together to produce a single checksum byte. Replace the lowest byte of `bridge` with this checksum.

### Input

- Line 1: integer $T$ — number of test cases.
- Next $T$ lines: each contains a 32-bit unsigned integer in hexadecimal format (with `0x` prefix).

### Output

For each test case, output the result as a zero-padded 8-digit uppercase hexadecimal value with `0x` prefix (e.g., `0x0A2B4C6D`).

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
A mysterious string is given in the format `d_1|d_2|...|d_m`, where each $d_i$ is a decimal integer. To decode it:
1. Convert each $d_i$ to its uppercase hexadecimal string representation with **no zero-padding** (e.g., `20065` → `4E61`).
2. Concatenate all hex strings sequentially to form the working string $S$.

**Queries (two types):**
- `Insert idx str` — Decode `str` (in mysterious string format) into a hex string $S_\text{ins}$ using the same Initialization rules. Insert $S_\text{ins}$ into $S$ at position `idx` (0-indexed, inserting before the character currently at `idx`).
- `Remove idx len` — Delete exactly `len` characters from $S$ starting at position `idx` (remove characters at indices `idx` through `idx + len - 1`).

After all queries, decode the final $S$ into ASCII: every consecutive pair of hex digits represents one ASCII character (e.g., `4E6F` → `No`).

### Input

- Line 1: the initial mysterious string in `d_1|d_2|...|d_m` format.
- Line 2: integer $q$ ($1 \leq q \leq 100$).
- Next $q$ lines: query descriptions as specified above.

### Constraints

- $0 \leq$ `idx` $\leq |S|$ for Insert; $0 \leq$ `idx`, `idx + len - 1` $< |S|$ for Remove.
- The length of $S$ never exceeds 10,000 characters at any point.

### Output

Output the final decoded ASCII string on a single line.

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

A billboard system stores text as a one-dimensional array of $N$ strings. Each string $i$ has a corresponding length value `len[i]` that specifies how many characters are currently in it. The strings do **not** use null terminators; `len[i]` is the sole indicator of length. All strings are initially empty.

You must process $Q$ queries of the following types:

- `0 i c` — Append the single character `c` to the end of string $i$.
- `1 i s` — Append the string `s` to the end of string $i$.
- `2` — Print all non-empty strings in order from index $0$ to $N-1$, one per line.

### Input

- Line 1: two integers $N$ ($1 \leq N \leq 100$) and $Q$ ($1 \leq Q \leq 1000$).
- Next $Q$ lines: queries as described above.

### Output

For every type-`2` query, print all non-empty strings in index order, each on its own line.

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

There are $N$ spires arranged in a line. Each spire $i$ has a height $H[i]$ and broadcasts energy $V[i]$ to its immediate neighbours (left: $i-1$, right: $i+1$). A neighbour **absorbs** the energy only if it is **strictly taller** than spire $i$. If the neighbour is shorter, equal in height, or does not exist, the energy dissipates.

Find the spire that receives the most total absorbed energy. If multiple spires are tied, output the one with the **smaller index** (0-indexed).

### Input

- Line 1: integer $N$ ($1 \leq N \leq 5 \times 10^6$).
- Next $N$ lines: two integers $H[i]$ (height, $|H[i]| \leq 2 \times 10^9$) and $V[i]$ (energy broadcast, $0 \leq V[i] \leq 10^9$).

### Output

Two space-separated integers on one line: the 0-indexed spire number that receives maximum energy, and the total energy it receives.

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

Fujiwara no Sai is analyzing chess puzzles. He wants to know if it is possible to capture the opponent's King in **2 moves or fewer**.

The chessboard is a standard 8×8 grid, columns labeled `A`–`H`, rows labeled `1`–`8`. Sai controls the following pieces using standard chess movement rules:

| Piece | Movement |
|-------|----------|
| Q (Queen) | Any number of vacant squares horizontally, vertically, or diagonally |
| R (Rook) | Any number of vacant squares horizontally or vertically |
| N (Knight) | L-shape: 2 squares in one direction, 1 square perpendicular |
| B (Bishop) | Any number of vacant squares diagonally |
| P (Pawn) | Moves forward one square; captures diagonally forward one square; may move two squares from starting row |
| K (King) | One square in any direction |

A piece can **capture** an opponent's piece by moving to its square. Sai wins by capturing the opponent's King.

### Input

- Line 1: integer $t$ — number of test cases.
- For each test case:
  - Line 1: integer $p$ — number of Sai's pieces.
  - Next $p$ lines: piece type and position (e.g., `Q H1`).
  - Line: integer $o$ — number of opponent's pieces.
  - Next $o$ lines: opponent piece type and position.

### Output

For each test case:
- If the King can be captured in 1 move: print `1`, then the move on the next line (start and end position separated by a space, e.g., `H1 H8`).
- If the King can be captured in 2 moves: print `2`, then both moves on separate lines.
- If the King cannot be captured in 2 moves or fewer: print `0`.

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

$n$ mages are undergoing the First-Class Mage Exam. Each mage has a name and a mana value (an integer from $-99$ to $99$). You are given $q$ queries; each query specifies a sorting rule. Sort all mages by the specified rule and output their names.

**Sorting Rules:**

| Rule | Sort key | Tiebreaker |
|------|----------|------------|
| 0 | `value` ascending | Name lexicographically ascending |
| 1 | `value²` ascending | Name lexicographically ascending |
| 2 | Digit sum of `|value|` ascending (e.g., $-85 \to 8+5=13$) | Name lexicographically ascending |
| 3 | Swap tens and units digits of `value`, preserve sign, sort ascending (e.g., $78 \to 87$, $-6 \to -60$, $10 \to 1$) | Name lexicographically ascending |
| 4 | Absolute value is prime → comes first; non-prime → comes after. Within each group, sort by original `value` ascending. | Name lexicographically ascending |

### Input

- Line 1: two integers $n$ and $q$ ($1 \leq n, q \leq 100$).
- Next $n$ lines: `name` (at most 30 characters) and `value` ($-99 \leq \text{value} \leq 99$).
- Next $q$ lines: a single integer (0–4) — the rule for that query.

### Output

For each query, output the sorted names one per line, followed by a blank line.

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
