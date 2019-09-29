Based on discussion from https://www.reddit.com/r/rust/comments/d9zfa6/how_come_php_seems_so_much_faster_than_rust/


## Run

```bash
$ cargo install hyperfine
$ ./genearate_log.sh
$ cargo build --release --bins
$ hyperfine 'target/release/original < test.log'
$ hyperfine 'target/release/output_buffered < test.log'
$ hyperfine 'target/release/output_and_input_buffered < test.log'
$ hyperfine 'target/release/parser < test.log'
```

## Results

``` bash
$ hyperfine --warmup 10 'target/release/original < test.log'
Benchmark #1: target/release/original < test.log
  Time (mean ± σ):     151.0 ms ±   1.3 ms    [User: 53.2 ms, System: 89.6 ms]
  Range (min … max):   148.9 ms … 153.8 ms    19 runs

$ hyperfine --warmup 10 'target/release/output_buffered < test.log'
Benchmark #1: target/release/output_buffered < test.log
  Time (mean ± σ):      39.4 ms ±   0.7 ms    [User: 32.2 ms, System: 4.1 ms]
  Range (min … max):    38.4 ms …  42.4 ms    68 runs

$ hyperfine --warmup 10 'target/release/output_buffered2 < test.log'
Benchmark #1: target/release/output_buffered2 < test.log
  Time (mean ± σ):      84.4 ms ±   2.6 ms    [User: 73.5 ms, System: 8.3 ms]
  Range (min … max):    81.6 ms …  94.9 ms    34 runs

$ hyperfine --warmup 10 'target/release/output_and_input_buffered < test.log'
Benchmark #1: target/release/output_and_input_buffered < test.log
  Time (mean ± σ):      40.4 ms ±   0.9 ms    [User: 34.9 ms, System: 3.8 ms]
  Range (min … max):    39.0 ms …  43.5 ms    64 runs

$ hyperfine --warmup 10 'target/release/parser < test.log'
Benchmark #1: target/release/parser < test.log
  Time (mean ± σ):     246.1 ms ±   1.6 ms    [User: 68.6 ms, System: 174.6 ms]
  Range (min … max):   244.2 ms … 250.0 ms    12 runs
```

## Links

- https://www.reddit.com/r/rust/comments/9xedap/how_to_achieve_fast_stdinstdout_io_suitable_for/e9t4vle/
- output buffered example https://www.reddit.com/r/rust/comments/8833lh/performance_of_parsing_large_file_2gb/dwj6ozo/
- https://stackoverflow.com/questions/31289588/converting-a-str-to-a-u8
- for `write_fmt` use `write!` https://stackoverflow.com/questions/32472495/how-do-i-write-a-formatted-string-to-a-file/32472705#32472705
