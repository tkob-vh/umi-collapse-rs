# ASC25-RNAm5C

## Environment
Some python dependencies:
- `cutseq`
- `polars`
- `snakemake`
- `scipy`
Run `conda env create -f environment.yml` to create the conda environment.


## build
Run the `build.sh`

## Reference Indexes Construction
Download the reference dataset for [dna](https://ftp.ensembl.org/pub/release-113/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz) and [ncrna](https://ftp.ensembl.org/pub/release-113/fasta/homo_sapiens/ncrna/Homo_sapiens.GRCh38.ncrna.fa.gz)
Then run the `index.sh`
