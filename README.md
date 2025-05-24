# umi-collapse-rs
Rewrite [UMICollapse](https://github.com/Daniel-Liu-c0deb0t/UMICollapse) in Rust, achieving `~4.5x` speedup (done in [ASC25](https://www.asc-events.net/StudentChallenge/History/2025/index.html)).

## Usage
```sh
#!/bin/bash

numactl --cpunodebind=0 --membind=0 ./umicollapse/target/release/umicollapse --mode bam \
  --data naive \
  --merge avgqual \
  --num-threads 16 \
  -i "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" \
  -o "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam"

```


## Optimization Methods
To be finished...
