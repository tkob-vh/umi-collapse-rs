#!/bin/bash
#SBATCH -w hepnode[0-3]
#SBATCH -N 1
#SBATCH -n 64
#SBATCH -o output/%j.out

srr=SRR23538290
datadir=$HOME/ASC25-RNAm5C/data
basedir=$(pwd)

./target/release/umicollapse --mode bam \
  --data naive \
  --merge avgqual \
  --num-threads 64 \
  -i "${datadir}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" \
  -o "${datadir}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup_rs.bam"

# vtune -collect hotspots \
#   -data-limit=0 \
#   -source-search-dir=$basedir \
#   -search-dir=$basedir \
#   -result-dir=$basedir/profiling_results \
#   ./target/debug/umicollapse --mode bam \
#   --data naive \
#   --merge avgqual \
#   -i "${datadir}/${srr}.mRNA.genome.mapped.sorted.bam" \
#   -o "${datadir}/${srr}.mRNA.genome.mapped.sorted.dedup.bam"
