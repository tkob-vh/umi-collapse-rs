#!/bin/bash

./samtools-1.21/samtools view --threads "${num_threads}" \
  --expr "[XM] * 20 <= (qlen-sclen) && [Zf] <= 3 && 3 * [Zf] <= [Zf] + [Yf]" \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam" \
  --output-fmt BAM \
  --output "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.filtered.bam"
