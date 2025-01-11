#!/bin/bash

./samtools-1.21/samtools view --threads "${num_threads}" \
  --exclude-flags 3980 \
  --count \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" >"${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam.tsv"
