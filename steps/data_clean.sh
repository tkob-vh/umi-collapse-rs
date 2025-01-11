#!/bin/bash

cutseq "${datadir}/${srr}/${srr}.fastq" \
  --threads "${num_threads}" \
  --adapter-name INLINE \
  --min-length 20 \
  --trim-polyA \
  --ensure-inline-barcode \
  --output-file "${dataout}/${srr}/${srr}.fastq_cut" \
  --short-file "${dataout}/${srr}/${srr}.fastq_tooshort" \
  --untrimmed-file "${dataout}/${srr}/${srr}.fastq_untrimmed"
