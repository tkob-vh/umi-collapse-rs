#!/bin/bash

java -server -Xms8G -Xmx40G -Xss100M -Djava.io.tmpdir="${dataout}/${srr}" \
  -jar ./UMICollapse-1.0.0/umicollapse.jar bam \
  -t 2 -T 16 \
  --data naive \
  --merge avgqual \
  --two-pass \
  -i "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" \
  -o "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam" >"${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.log"
