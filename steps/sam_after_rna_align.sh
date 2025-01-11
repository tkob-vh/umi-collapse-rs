#!/bin/bash

./samtools-1.21/samtools fastq --threads "${num_threads}" \
  -O "${dataout}/${srr}/${srr}.ncrna.unmapped.bam" >"${dataout}/${srr}/${srr}.mRNA.fastq"
