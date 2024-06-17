#!/bin/bash

# Execute job in the partition "lva" unless you have special requirements.
#SBATCH --partition=lva
# Name your job to be able to identify it later
#SBATCH --job-name FibIter
# Redirect output stream to this file
#SBATCH --output=fib_iter.log
# Maximum number of tasks (=processes) to start in total
#SBATCH --ntasks=1
# Maximum number of tasks (=processes) to start per node
#SBATCH --ntasks-per-node=1
# Enforce exclusive node allocation, do not share with other jobs
#SBATCH --exclusive

valgrind --cache-sim=no --branch-sim=no --tool=cachegrind --cachegrind-out-file=fib_iter.cg ./src/lua ./fib_iter.lua

