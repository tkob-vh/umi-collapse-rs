#!/bin/bash

numactl --cpunodebind=0 --membind=0 ./umicollapse/target/release/umicollapse --mode bam \
  --data naive \
  --merge avgqual \
  --num-threads 16 \
  -i "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" \
  -o "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam"
