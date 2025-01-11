#!/bin/bash

./samtools-1.21/samtools sort --threads "${num_threads}" \
  --write-index \
  --output-fmt BAM \
  -o "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.bam"
