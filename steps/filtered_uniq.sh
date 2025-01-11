#!/bin/bash

./samtools-1.21/samtools view --expr "rlen<100000" \
  --with-header "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.filtered.bam" |
  ./hisat-3n/hisat-3n-table --threads "${num_threads}" \
    --unique-only \
    --alignments - \
    --ref "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa" \
    --output-name /dev/stdout \
    --base-change C,T |
  cut -f 1,2,3,5,7 |
  gzip -c >"${dataout}/${srr}/${srr}_filtered_uniq.tsv.gz"
