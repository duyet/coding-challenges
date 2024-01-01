# Write Your Own wc Tool

https://codingchallenges.fyi/challenges/challenge-wc

Build and Test:

```bash
cargo build
cargo test
```

Compared the rust version with the `wc`

```bash
❯ ./target/debug/wc -w test.txt && wc -w test.txt  

   58164 test.txt
   58164 test.txt
```

```bash
❯ ./target/debug/wc -c test.txt && wc -c test.txt

  342190 test.txt
  342190 test.txt
```

```bash
❯ ./target/debug/wc -cw test.txt && wc -cw test.txt

   58164  342190 test.txt
   58164  342190 test.txt
```

```bash
❯ ./target/debug/wc test.txt && wc test.txt    

    7145   58164  342190 test.txt
    7145   58164  342190 test.txt
```

```bash
❯ cat test.txt | cargo r  && cat test.txt | wc   

    7145   58164  342190 
    7145   58164  342190

```
