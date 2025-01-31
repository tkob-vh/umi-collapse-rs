# ASC25-RNAm5C

## Environment Setup
Some python dependencies:
- `cutseq`
- `polars`
- `snakemake`
- `scipy`

Run `conda env create -f environment.yml` to create the conda environment.

### RUST
Also, you need to install `RUST`:
```bash
curl https://sh.rustup.rs -sSf | sh
```                                                                                                                                                                                                             
## Build
Run the `build.sh` to build `hisat-3n`, `samtools` and `umicollapse`.

## Reference Indexes Construction
> [!NOTE]
> You can just skip this step if you have already downloaded the data and build the indexes.

Download the reference dataset for [dna](https://ftp.ensembl.org/pub/release-113/fasta/homo_sapiens/dna/Homo_sapiens.GRCh38.dna.primary_assembly.fa.gz) and [ncrna](https://ftp.ensembl.org/pub/release-113/fast
a/homo_sapiens/ncrna/Homo_sapiens.GRCh38.ncrna.fa.gz)

Then run the `index.sh`.



## Running the workflow
After build, prepare the `conda` venv and run the workflow.
```bash
conda activate RNAm5C # replace it with you venv name
snakemake # add `--benchmark-extended` flag for more benchmark info
```
By default, all the results and intermediate files locates at `./workspace`. Also, the benchmark files are generated under `./workspace/benchmark`. The log file is located at `./workspace/.snakemake/log`.

If you don't want to keep last run files, just use `rm -r ./workspace`.

By default, the workflow needs a slurm system. (see `./profiles/default/config.yaml`) The slurm output is under `./output`.


## Test
You don't have to run through the entire workflow everytime. The separate steps in the workflow are placed in dir `steps`, and you can invoke them using `snakeless_1.sh` (Comment out the unnecessary steps in it before you run).

## Profile
The `vtune.sh` is used to run vtune and profile the apps.

The main steps are stored in dir `steps`, and `vtune.sh` calls them in it.

You can define the vtune type (such as `hotspots` `performance-snapshot`) as follows:
```sh
vtune_type=hotspots sbatch vtune.sh
```
The default vtune type is `hotspots`.

It's often the case that we don't need to profile every step in one job, so modify the script before you run it.

## Validation
The `./validate.sh` is used to verify the result.

The `<SRR>.filtered.tsv` must be present under `<your workspace>/<SRR>` directory. (`SRR` is any of `SRR23538290`, `SRR23538291`, `SRR23538292`)

By default, the validation need reference files located at `datasets/workspaces-ref/`, where `datasets` is a symbolic link somewhere in our Heptagon SCC Team cluster.

While verifying, two temporary files named `true_ur.txt` and `detected_ur.txt` will be created, after verifying they will be deleted.

```bash
./validate.sh <your workspace>
```

## Clean workspace
The `./clean_workspace.sh` is used to clean the intermediate files to save storage space.
```bash
./clean_workspace.sh <your workspace>
```
After cleaning, the workspace will only contain the `<SRR>.filtered.tsv` files(used for validation), some log files;
bam, fastq, sam files and other intermediate files will be deleted.

