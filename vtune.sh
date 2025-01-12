#!/bin/bash
#SBATCH -J vtune
#SBATCH -N 1
#SBATCH -n 48
#SBATCH -w hepnode0
#SBATCH -o output/vtune.%j.log
#SBATCH --time=03:00:00

set -euo pipefail

export basedir=$(pwd)
export datadir=/data/Competitions/ASC25/m5C/data
export dataout="${basedir}/data"
export num_threads=32
export srr=${srr:-SRR23538290}
vtune_type=${vtunt_type:-"performance-snapshot"}
profiling_results=/data/Competitions/ASC25/m5C/profiling_results

eval $(spack load --sh intel-oneapi-vtune@2024.0.1)

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

# echo "====== data_clean: $(date) ======"
# time data_clean

echo "====== rna_align ======"
vtune -collect "${vtune_type}" \
  -data-limit=0 \
  -source-search-dir="${basedir}/hisat-3n" \
  -source-search-dir="${basedir}/samtools-1.21" \
  -search-dir="${basedir}/hisat-3n" \
  -search-dir="${basedir}/samtools-1.21" \
  -result-dir="${profiling_results}/rna_align/vtune_${vtune_type}_${SLURM_JOBID}" \
  -- ./steps/rna_align.sh

# echo "====== sam_after_rna_align ======"
# time sam_after_rna_align

echo "====== genome_align ======"
vtune -collect "${vtune_type}" \
  -data-limit=0 \
  -source-search-dir="${basedir}/hisat-3n" \
  -source-search-dir="${basedir}/samtools-1.21" \
  -search-dir="${basedir}/hisat-3n" \
  -search-dir="${basedir}/samtools-1.21" \
  -result-dir="${profiling_results}/genome_align/vtune_${vtune_type}_${SLURM_JOBID}" \
  -- ./steps/genome_align.sh

# echo "====== sam_sort ======"
# time sam_sort

# echo "====== stat_mapping_num ======"
# time stat_mapping_num

echo "====== dedup_mapping ======"
vtune -collect "${vtune_type}" \
  -data-limit=0 \
  -source-search-dir="${basedir}/UMICollapse-1.0.0" \
  -search-dir="${basedir}/UMICollapse-1.0.0" \
  -result-dir="${profiling_results}/dedup_mapping/vtune_${vtune_type}_${SLURM_JOBID}" \
  -- ./steps/dedup_mapping.sh

# echo "====== dedup_index ======"
# time dedup_index

echo "====== unifiltered_unique ======"
vtune -collect "${vtune_type}" \
  -data-limit=0 \
  -source-search-dir="${basedir}/samtools-1.21" \
  -source-search-dir="${basedir}/hisat-3n" \
  -search-dir="${basedir}/hisat-3n" \
  -search-dir="${basedir}/samtools-1.21" \
  -result-dir="${profiling_results}/unfiltered_uniq/vtune_${vtune_type}_${SLURM_JOBID}" \
  -- ./steps/unfiltered_uniq.sh

# echo "====== unfiltered_multi ======"
# unfiltered_multi

# echo "====== filtering ======"
# time filtering

# echo "====== get filtered_uniq ======"
# filtered_uniq

# echo "====== filtered_multi ======"
# filtered_multi

# echo "====== join_pileup ======"
# time join_pileup

echo "====== Done: $(date) ======"
