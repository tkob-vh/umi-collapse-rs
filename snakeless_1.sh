#!/bin/bash
#SBATCH -J nosnake
#SBATCH -N 1
#SBATCH -n 32
#SBATCH -w hepnode[1-3]
#SBATCH -o output/no_snake.%j.log

set -euo pipefail

export basedir=$(pwd)
export datadir=/data/Competitions/ASC25/m5C/data
export dataout="${basedir}/data"
export num_threads=32
export srr=${srr:-SRR23538290}

if [ -z "$CONDA_DEFAULT_ENV" ]; then
  conda activate m5C-venv
fi

echo "================= Debug Info ================="
echo "Job Owner: $(whoami)"
echo "Job Nodelist: ${SLURM_JOB_NODELIST}"
echo "Job Date: $(date)"
echo "Current Branch: $(git rev-parse --abbrev-ref HEAD)"
echo "Current Commit ID: $(git rev-parse HEAD)"
echo "SRR ID: ${srr}"
echo
echo "+++ Job Content +++"
cat $0
echo "+++ Job Content +++"
echo -e "================== Debug Info ==================\n\n"

if [ ! -d "${dataout}" ]; then
  mkdir "data"
fi
if [ ! -d "${dataout}/${srr}" ]; then
  mkdir "data/${srr}"
fi

## Only process one srr file.
## The following commands should be executed for each dataset.

echo "====== data_clean: $(date) ======"
time ./steps/data_clean.shdata_clean

echo "====== rna_align ======"
time ./steps/rna_align.sh

echo "====== sam_after_rna_align ======"
time ./steps/sam_after_rna_align.sh

echo "====== genome_align ======"
time ./steps/genome_align.sh

echo "====== sam_sort ======"
time ./steps/sam_sort.sh

echo "====== stat_mapping_num ======"
time ./steps/stat_mapping_num.sh

echo "====== dedup_mapping ======"
time ./steps/dedup_mapping.sh

echo "====== dedup_index ======"
time ./steps/dedup_index.sh

echo "====== unifiltered_unique ======"
time ./steps/unfiltered_uniq.sh

echo "====== unfiltered_multi ======"
time ./steps/unfiltered_multi.sh

echo "====== filtering ======"
time ./steps/filtering.sh

echo "====== get filtered_uniq ======"
time ./steps/filtered_uniq.sh

echo "====== filtered_multi ======"
time ./steps/filtered_multi.sh

echo "====== join_pileup ======"
time ./steps/join_pileup.sh

echo "====== Done: $(date) ======"
