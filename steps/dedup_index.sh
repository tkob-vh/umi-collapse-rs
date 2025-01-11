#!/bin/bash

./samtools-1.21/samtools index --threads "${num_threads}" \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam" \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam.bai"
