# RustMpiMandelbrot
An experimental project using Rust &amp; OpenMPI to render the Mandelbrot set

[Follow this guide to get slurm & openMpi setup on your cluster](https://glmdev.medium.com/building-a-raspberry-pi-cluster-784f0df9afbd)

```bash
cargo build --release
sbatch ./run.sh
squeue
