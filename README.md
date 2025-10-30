# qdo

This project is designed to solve several problems that I run into regularly, namely:

1. I work on clusters managed by both Slurm and PBS Pro, and can forget to context switch
2. I tend to submit many one-off jobs that are similar, but not enough to simply use array jobs
3. I am terrible at managing job scripts, log files, and outputs.
4. I like Rust, but do not have a good reason to write it for my regular work.

If you think about it, these four problems really collapse into two problems:

1. I am bad at HPC
2. I like Rust

Because this is the bad place, I still have to write Python and C++ if I want to do HPC work (for now). But a Rust-based CLI that interfaces with the scheduler and manages my outputs for me? Say less.

