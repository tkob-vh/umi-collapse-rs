#!/bin/bash

./hisat-3n/hisat-3n --index "${datadir}/ncrna_ref/Homo_sapiens.GRCh38.ncrna.fa" \
  --summary-file "${dataout}/${srr}/map2ncrna.output.summary" \
  --new-summary -q \
  -U "${dataout}/${srr}/${srr}.fastq_cut" \
  --threads "${num_threads}" \
  --base-change C,T \
  --mp 8,2 \
  --no-spliced-alignment \
  --directional-mapping |
  ./samtools-1.21/samtools view --threads "${num_threads}" \
    --expr '!flag.unmap' \
    --output-fmt BAM \
    --unoutput "${dataout}/${srr}/${srr}.ncrna.unmapped.bam" \
    --output "${dataout}/${srr}/${srr}.ncrna.mapped.bam"
