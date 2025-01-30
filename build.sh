#!/bin/bash
#SBATCH -w hepnode0 ## Only run it on hepnode0
#SBATCH -N 1
#SBATCH -n 16
#SBATCH -J m5C-build
#SBATCH -o output/build.%j.out

set -euo pipefail

echo "================= Debug Info ================="
echo "Job Owner: $(whoami)"
echo "Job Nodelist: ${SLURM_JOB_NODELIST:-{'not a slurm job'}"
echo "Job Date: $(date)"
echo "Current Branch: $(git rev-parse --abbrev-ref HEAD)"
echo "Current Commit ID: $(git rev-parse HEAD)"
echo
echo "+++ Job Content +++"
cat $0
echo "+++ Job Content +++"
echo -e "================== Debug Info ==================\n\n"

eval $(spack load --sh intel-oneapi-compilers@2024)
eval $(spack load --sh intel-tbb)

# build hisat-3n
pushd hisat-3n

make -j16

popd

# build samtools
pushd samtools-1.21

if [ ! -e configure ]; then
  autoreconf
  ./configure --enable-configure-htslib
fi

make -j16
# make prefix=./ install

popd

# UMICollapse
eval $(spack unload --sh gcc@13.2.0)
pushd umicollapse
cargo build --release
popd

# pushd UMICollapse-1.0.0
#
# if [ ! -d lib ]; then
#     mkdir lib
#     pushd lib
#     curl -O -L https://repo1.maven.org/maven2/com/github/samtools/htsjdk/2.19.0/htsjdk-2.19.0.jar
#     curl -O -L https://repo1.maven.org/maven2/org/xerial/snappy/snappy-java/1.1.7.3/snappy-java-1.1.7.3.jar
#     popd
# fi
#
# popd
