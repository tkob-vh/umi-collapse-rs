#!/bin/bash
./hisat-3n/hisat-3n --index "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa" \
  --threads "${num_threads}" \
  --summary-file "${dataout}/${srr}/map2genome.output.summary" \
  --new-summary -q \
  -U "${dataout}/${srr}/${srr}.mRNA.fastq" \
  --directional-mapping \
  --base-change C,T \
  --pen-noncansplice 20 \
  --mp 4,1 |
  ./samtools-1.21/samtools view --threads "${num_threads}" \
    --expr '!flag.unmap' \
    --output-fmt BAM \
    --unoutput "${dataout}/${srr}/${srr}.mRNA.genome.unmapped.bam" \
    --output "${dataout}/${srr}/${srr}.mRNA.genome.mapped.bam"
