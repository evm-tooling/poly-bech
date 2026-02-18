# Non-linear scaling benchmarks

Five benchmark suites, each with **10 tests** and increasing input size. Use these to get line charts that show clear **non-linear** time vs size (not flat like hashing).

| File | Algorithm | Complexity | Sizes |
|------|-----------|------------|--------|
| `sort.bench` | Stdlib sort (int32 array) | O(n log n) | 100 … 1000 |
| `concat.bench` | Naive `s += "x"` in a loop | O(n²) | 100 … 1000 |
| `bubble.bench` | Bubble sort | O(n²) | 100 … 1000 |
| `matmul.bench` | Naive n×n matrix multiply | O(n³) | 8, 16, … 80 |
| `regex.bench` | Pattern `(a+)+b` on `"a"*n+"b"` | backtracking | 10, 20, … 100 |

## Run one suite

From repo root:

```bash
poly-bench run examples/demo-basic/benchmarks/nonlinear/sort.bench --output examples/demo-basic/out/
```

Charts are written to the `--output` directory (e.g. `sort-line.svg`, `concat-line.svg`, …).

## Fixtures

- **sort / bubble:** `sort_100.hex` … `sort_1000.hex` — big-endian int32 arrays (4×n bytes).
- **concat:** `n_100.hex` … `n_1000.hex` — single uint32 (4 bytes, little-endian).
- **matmul:** `mat_8.hex` … `mat_80.hex` — single uint32 n (4 bytes).
- **regex:** `re_10.hex` … `re_100.hex` — single uint32 length (4 bytes).

All fixture paths are relative to this directory.
