#!/bin/bash

# java -server -Xms80G -Xmx150G -Xss100M -Djava.io.tmpdir="${dataout}/${srr}" \
#   -jar ./UMICollapse-1.0.0/umicollapse.jar bam \
#   --data naive \
#   --merge avgqual \
#   -i "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" \
#   -o "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam" >"${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.log"
./umicollapse/target/release/umicollapse --mode bam \
  --data naive \
  --merge avgqual \
  --num-threads 64 \
  -i "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" \
  -o "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam"
