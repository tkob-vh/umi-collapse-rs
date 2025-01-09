#!/bin/bash
#SBATCH -J nosnake
#SBATCH -N 1
#SBATCH -n 32
#SBATCH -w hepnode[1-3]
#SBATCH -o output/no_snake.%j.log

set -euo pipefail

basedir=$(pwd)
datadir=/data/Competitions/ASC25/m5C/data
dataout="${basedir}/data"
num_threads=32
srr=${srr:-SRR23538290}

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

echo "====== data cleaning: $(date) ======"
cutseq "${datadir}/${srr}/${srr}.fastq" \
  --threads "${num_threads}" \
  --adapter-name INLINE \
  --min-length 20 \
  --trim-polyA \
  --ensure-inline-barcode \
  --output-file "${dataout}/${srr}/${srr}.fastq_cut" \
  --short-file "${dataout}/${srr}/${srr}.fastq_tooshort" \
  --untrimmed-file "${dataout}/${srr}/${srr}.fastq_untrimmed"

echo "====== get ncrna.mapped.bam: $(date) ======"
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

echo "====== get mRNA.fastq: $(date) ======"
./samtools-1.21/samtools fastq --threads "${num_threads}" \
  -O "${dataout}/${srr}/${srr}.ncrna.unmapped.bam" >"${dataout}/${srr}/${srr}.mRNA.fastq"

echo "====== get mRNA.genome.mapped.bam: $(date) ======"
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

echo "====== get mRNA.genome.mapped.sorted.bam: $(date) ======"
./samtools-1.21/samtools sort --threads "${num_threads}" \
  --write-index \
  --output-fmt BAM \
  -o "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.bam"

echo "====== get mRNA.genome.mapped.sorted.bam.tsv: $(date) ======"
./samtools-1.21/samtools view --threads "${num_threads}" \
  --exclude-flags 3980 \
  --count \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" >"${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam.tsv"

echo "====== get mRNA.genome.mapped.sorted.dedup.bam: $(date) ======"
java -server -Xms8G -Xmx40G -Xss100M -Djava.io.tmpdir="${dataout}/${srr}" \
  -jar ./UMICollapse-1.0.0/umicollapse.jar bam \
  -t 2 -T 16 \
  --data naive \
  --merge avgqual \
  --two-pass \
  -i "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.bam" \
  -o "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam" >"${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.log"

echo "====== get mRNA.genome.mapped.sorted.dedup.bam.bai: $(date) ======"
./samtools-1.21/samtools index --threads "${num_threads}" \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam" \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam.bai"

echo "====== get unifiltered_unique.tsv.gz: $(date) ======"
./samtools-1.21/samtools view --expr "rlen<100000" \
  --with-header "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam" |
  ./hisat-3n/hisat-3n-table --threads "${num_threads}" \
    --unique-only \
    --alignments - \
    --ref "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa" \
    --output-name /dev/stdout \
    --base-change C,T |
  cut -f 1,2,3,5,7 |
  gzip -c >"${dataout}/${srr}/${srr}_unfiltered_uniq.tsv.gz"

echo "====== get unfiltered_multi.tsv.gz: $(date) ======"
./samtools-1.21/samtools view --expr "rlen<100000" \
  --with-header "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam" |
  ./hisat-3n/hisat-3n-table --threads "${num_threads}" \
    --multiple-only \
    --alignments - \
    --ref "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa" \
    --output-name /dev/stdout \
    --base-change C,T |
  cut -f 1,2,3,5,7 |
  gzip -c >"${dataout}/${srr}/${srr}_unfiltered_multi.tsv.gz"

echo "====== get mRNA.genome.mapped.sorted.dedup.filtered.bam: $(date) ======"
./samtools-1.21/samtools view --threads "${num_threads}" \
  --expr "[XM] * 20 <= (qlen-sclen) && [Zf] <= 3 && 3 * [Zf] <= [Zf] + [Yf]" \
  "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.bam" \
  --output-fmt BAM \
  --output "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.filtered.bam"

echo "====== get filtered_uniq.tsv.gz: $(date) ======"
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

echo "====== get filtered_multi.tsv.gz: $(date) ======"
./samtools-1.21/samtools view --expr "rlen<100000" \
  --with-header "${dataout}/${srr}/${srr}.mRNA.genome.mapped.sorted.dedup.filtered.bam" |
  ./hisat-3n/hisat-3n-table --threads "${num_threads}" \
    --multiple-only \
    --alignments - \
    --ref "${datadir}/ref/Homo_sapiens.GRCh38.dna.primary_assembly.fa" \
    --output-name /dev/stdout \
    --base-change C,T |
  cut -f 1,2,3,5,7 |
  gzip -c >"${dataout}/${srr}/${srr}_filtered_multi.tsv.gz"

echo "====== join_pileup and get genome.arror: $(date) ======"
python ./m5C-UBSseq-0.1/bin/join_pileup.py -i \
  "${dataout}/${srr}/${srr}_unfiltered_uniq.tsv.gz" \
  "${dataout}/${srr}/${srr}_unfiltered_multi.tsv.gz" \
  "${dataout}/${srr}/${srr}_filtered_uniq.tsv.gz" \
  "${dataout}/${srr}/${srr}_filtered_multi.tsv.gz" \
  -o "${dataout}/${srr}/${srr}_genome.arrow"

echo -e "\n\nDone: $(date)"
