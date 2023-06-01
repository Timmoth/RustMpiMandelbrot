#!/bin/bash
#SBATCH --ntasks=28
cd $SLURM_SUBMIT_DIR

mpiexec -n 28 /nfs/computecluster/rust/test/mandel/target/release/mandel
