#!/bin/bash
#SBATCH -N 1
#SBATCH -n 32
#SBATCH -w hepnode0
#SBATCH -J m5C-index
#SBATCH -o output/index.%j.log

set -euo pipefail

basedir=$(pwd)
datadir=/data/Competitions/ASC25/m5C/data

# if [ ! -f "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa" ]; then
#     echo "Reference genome file not found!" >&2
#     exit 1
# fi
#
# ./hisat-3n/hisat-3n-build -p 32 --base-change C,T "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa" "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa"
# ./samtools-1.21/samtools faidx "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa"
#
# awk 'BEGIN{{OFS="\\t"}}{{print $1,$1,0,$2,"+"}}' "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa.fai" >"${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa.saf"

if [ ! -f "${datadir}/ncrna_ref/Homo_sapiens.GRCh38.ncrna.fa" ]; then
    echo "ncRNA reference genome file not found!" >&2
    exit 1
fi

./hisat-3n/hisat-3n-build -p 16 --base-change C,T "${datadir}/ncrna_ref/Homo_sapiens.GRCh38.ncrna.fa" "${datadir}/ncrna_ref/Homo_sapiens.GRCh38.ncrna.fa"
./samtools-1.21/samtools faidx "${datadir}/ncrna_ref/Homo_sapiens.GRCh38.ncrna.fa"
