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

The original program was written in Java ([UMICollapse](https://github.com/Daniel-Liu-c0deb0t/UMICollapse)). Our optimization approach includes the following key strategies:

### 1. RIIR
Through comprehensive performance analysis, we discovered that the bottleneck wasn't in the UMICollapse program itself, but in the I/O operations of the Java library **htsjdk** that it depends on. Further research ([PMC7931820](https://pmc.ncbi.nlm.nih.gov/articles/PMC7931820/)) revealed that htsjdk has a corresponding C library **htslib**, which supports parallel BAM file reading/writing (while htsjdk only supports single-threaded I/O). Therefore, rewriting was necessary to leverage these performance improvements.

### 2. Cached Hash Values
This optimization was already present in the original Java version. Since the program maintains a very large hash table, computing hash values for keys during various operations on hash table entries incurs significant overhead. We cache the hash value in a field of the key and use a flag bit to indicate whether the hash value needs to be recalculated.

### 3. memchr Replacing Regex
Analysis showed that regular expression matching was consuming significant time. By analyzing the characteristics of the haystack, we found that substring search could completely replace regex matching, providing substantial performance gains.

### 4. Arena Memory Allocation (bumpalo) Replacing ptmalloc
The HashMap's Drop overhead was extremely high, taking approximately 1 minute. By using arena allocation to uniformly manage all HashMap entries and releasing them collectively at the end of their lifecycle, we reduced this overhead to around 4 seconds.
