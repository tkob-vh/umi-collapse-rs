#!/bin/bash
#SBATCH -N 1
#SBATCH -n 8
#SBATCH -w hepnode[1-3]
#SBATCH -J join
#SBATCH -o output/snakeless_2.%j.log

basedir=$(pwd)
dataout="${basedir}/data"
datadir=/data/Competitions/ASC25/m5C/data

srr0=SRR23538290
srr1=SRR23538291
srr2=SRR23538292

if [ -z "$CONDA_DEFAULT_ENV" ]; then
  conda activate m5C-venv
fi

echo "================= Debug Info ================="
echo "Job Owner: $(whoami)"
echo "Job Nodelist: ${SLURM_JOB_NODELIST}"
echo "Job Date: $(date)"
echo "Current Branch: $(git rev-parse --abbrev-ref HEAD)"
echo "Current Commit ID: $(git rev-parse HEAD)"
echo
echo "+++ Job Content +++"
cat $0
echo "+++ Job Content +++"
echo -e "================== Debug Info ==================\n\n"

echo "====== group_pileup and get WT.arrow: $(date) ======"
python ./m5C-UBSseq-0.1/bin/group_pileup.py -i \
  "${dataout}/${srr0}/${srr0}_genome.arrow" \
  "${dataout}/${srr1}/${srr1}_genome.arrow" \
  "${dataout}/${srr2}/${srr2}_genome.arrow" \
  -o "${dataout}/WT.arrow"

echo "====== select_sites and get WT.prefilter.tsv: $(date) ======"
python ./m5C-UBSseq-0.1/bin/select_sites.py \
  -i "${dataout}/WT.arrow" \
  -o "${dataout}/WT.prefilter.tsv"

echo "====== filter_sites: $(date) ======"
python ./m5C-UBSseq-0.1/bin/filter_sites.py \
  -i "${dataout}/${srr0}/${srr0}_genome.arrow" \
  -m "${dataout}/WT.prefilter.tsv" \
  -b "${dataout}/${srr0}/${srr0}.gb.tsv" \
  -o "${dataout}/${srr0}/${srr0}.filtered.tsv" &

python ./m5C-UBSseq-0.1/bin/filter_sites.py \
  -i "${dataout}/${srr1}/${srr1}_genome.arrow" \
  -m "${dataout}/WT.prefilter.tsv" \
  -b "${dataout}/${srr1}/${srr1}.gb.tsv" \
  -o "${dataout}/${srr1}/${srr1}.filtered.tsv" &

python ./m5C-UBSseq-0.1/bin/filter_sites.py \
  -i "${dataout}/${srr2}/${srr2}_genome.arrow" \
  -m "${dataout}/WT.prefilter.tsv" \
  -b "${dataout}/${srr2}/${srr2}.gb.tsv" \
  -o "${dataout}/${srr2}/${srr2}.filtered.tsv" &

wait
echo "====== Done: $(date) ======"
