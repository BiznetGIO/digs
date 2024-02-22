# Benchmark

Benchmarking is done on a very quiet machine. Every other service and app are
stopped.
The input used in this benchmark is located in `benches/input`.

- [digs.py](#digspy)
- [digs.rs](#digsrs)
- [Summary](#summary)

## digs.py

App location:

```bash
~ via 🐍 v3.7.3 (global37)
❯ which digs
/home/user/.virtualenvs/global37/bin/digs
```

Try:

```bash
~ via 🐍 v3.7.3 (global37)
❯ digs example.net A
1.0.0.1 - Cloudflare
93.184.216.34
8.8.8.8 - Google
93.184.216.34
8.8.4.4 - Google 1
93.184.216.34
9.9.9.9 - Quad9 1
93.184.216.34
```

Result:

```bash
~ via 🐍 v3.7.3 (global37)
❯ hyperfine --warmup 3 'digs example.net A'
Benchmark #1: digs example.net A
  Time (mean ± σ):     507.2 ms ± 173.8 ms    [User: 175.8 ms, System: 17.1 ms]
  Range (min … max):   348.7 ms … 738.4 ms    10 runs


~ via 🐍 v3.7.3 (global37)took 7s
❯ hyperfine --warmup 3 'digs example.net A'
Benchmark #1: digs example.net A
  Time (mean ± σ):     519.8 ms ± 159.8 ms    [User: 153.8 ms, System: 16.7 ms]
  Range (min … max):   351.0 ms … 771.9 ms    10 runs
```

## digs.rs

App location:

```bash
~
❯ which digs
/home/user/.cargo/bin/digs
```

Try:

```bash
~
❯ digs example.net A
  A    example.net.    93.184.216.34
  A    example.net.    93.184.216.34
  A    example.net.    93.184.216.34
  A    example.net.    93.184.216.34
```

Result:

```bash
~
❯ hyperfine --warmup 3 'digs example.net A'
Benchmark #1: digs example.net A
  Time (mean ± σ):     328.0 ms ± 202.0 ms    [User: 2.6 ms, System: 3.8 ms]
  Range (min … max):   168.8 ms … 619.4 ms    15 runs

~ 
❯ hyperfine --warmup 3 'digs example.net A'
Benchmark #1: digs example.net A
  Time (mean ± σ):     304.7 ms ± 195.4 ms    [User: 2.5 ms, System: 4.1 ms]
  Range (min … max):   159.4 ms … 629.4 ms    17 runs
```

## Summary

Using `digs.1.toml` as input:

| Tool       | Command            | Time (mean) |
| ---------- | ------------------ | ----------- |
| digs.rs 🦀 | `digs example.net` | **304.7ms** |
| digs.py 🐍 | `digs example.net` | **507.2ms** |

Using `digs.2.toml` as input:

```bash
> # digs.rs
❯ hyperfine --warmup 3 --max-runs 10 'digs example.net A -f digs.2.toml'
Benchmark #1: digs example.net A -f digs.2.toml
  Time (mean ± σ):      1.321 s ±  0.032 s    [User: 6.4 ms, System: 7.7 ms]
  Range (min … max):    1.259 s …  1.350 s    10 runs
  
  
> # digs.py
❯ hyperfine --warmup 3 --max-runs 10 'digs example.net A -f digs.2.yaml'
Benchmark #1: digs example.net A -f digs.2.yaml
  Time (mean ± σ):      1.548 s ±  0.215 s    [User: 158.3 ms, System: 19.6 ms]
  Range (min … max):    1.356 s …  1.960 s    10 runs
```

| Tool       | Command            | Time (mean) |
| ---------- | ------------------ | ----------- |
| digs.rs 🦀 | `digs example.net` | **1.321s**  |
| digs.py 🐍 | `digs example.net` | **1.546s**  |
