# ASC25-RNAm5C

## Environment Setup
Some python dependencies:
- `cutseq`
- `polars`
- `snakemake`
- `scipy`

Run `conda env create -f environment.yml` to create the conda environment.


## Build
Run the `build.sh`.

## Reference Indexes Construction
Download the reference dataset for [dna](https://ftp.ensembl.org/pub/release-113/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz) and [ncrna](https://ftp.ensembl.org/pub/release-113/fasta/homo_sapiens/ncrna/Homo_sapiens.GRCh38.ncrna.fa.gz)

Then run the `index.sh`.

## Profile
The `vtune.sh` is used to run vtune and profile the apps.
The main steps are stored in dir `steps`, and `vtune.sh` calls them in it.
You can define the vtune type (such as `hotspots` `performance-snapshot`) as follows:
```sh
vtune_type=hotspots sbatch vtune.sh
```
The default vtune type is `performance-snapshot`.
It's often the case that we don't need to profile every step in one job, so modify the script before you run it.


