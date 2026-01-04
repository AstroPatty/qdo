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


## Design

### Allocations

An allocation represents a location where you peform work. You start with a single allocation, your home directory. You can add additional allocations with `qdo init`. An allocation *may* be synonymous with an allocation you hold on the HPC system, but does not have to be. 

### Projects

Projects represent a collection of work that is logically connected. Projects are located inside an allocation, but projects names must be unique across *all* allocations. Projects may contain other projects. Nested projects are delimited with slashes. For example, you could create a "my_project/run" and a "my_project/debug" project.

### Resources

Resources contain data that may be useful to more than one job or project. Resources are located inside of projects. Common resources include job scripts, which can be templated.

### Jobs

Jobs represent units of work that are submitted to a scheduler. Jobs produce artifacts (e.g. logs or output), and have at least one input (a job script). Job names do not need to be unique within a project. If a job is submitted with the same name as another job, it superseeds the previous job. Superseeded jobs are still accessible, but accessing a job by name only will always access the most recent version.

#### Artifacts

Jobs produce artifacts, including logs and output.

