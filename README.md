# RustMpiMandelbrot
An experimental project using Rust &amp; OpenMPI to render the Mandelbrot set

[Follow this guide to get slurm & openMpi setup on your cluster](https://glmdev.medium.com/building-a-raspberry-pi-cluster-784f0df9afbd)

```bash
cargo build --release
sbatch ./run.sh
squeue
```

<p align="center">
   <div style="width:640;height:320">
       <img style="width: inherit" src="https://raw.githubusercontent.com/Timmoth/RustMpiMandelbrot/c94d6152e89a9920e19cce95fbb9bdfef71dccd0/images/fractal-2240x2240-200000000000000-5000.png">
</div>
</p>

<p align="center">
   <div style="width:640;height:320">
       <img style="width: inherit" src="https://github.com/Timmoth/RustMpiMandelbrot/blob/c94d6152e89a9920e19cce95fbb9bdfef71dccd0/images/fractal-2240x2240-200000000000000-8000.png?raw=true">
</div>
</p>
