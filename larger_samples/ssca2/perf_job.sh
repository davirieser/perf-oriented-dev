#!/bin/bash

# Execute job in the partition "lva" unless you have special requirements.
#SBATCH --partition=lva
# Name your job to be able to identify it later
#SBATCH --job-name Perf 
# Redirect output stream to this file
#SBATCH --output=timing_no_perf.log
# Maximum number of tasks (=processes) to start in total
#SBATCH --ntasks=1
# Maximum number of tasks (=processes) to start per node
#SBATCH --ntasks-per-node=1
# Enforce exclusive node allocation, do not share with other jobs
#SBATCH --exclusive

# /usr/bin/time perf stat -e L1-dcache-load-misses,L1-dcache-loads,L1-dcache-prefetch-misses,L1-dcache-prefetches,L1-dcache-store-misses,L1-dcache-stores ./ssca2 17
# /usr/bin/time perf stat -e L1-icache-load-misses,L1-icache-loads,LLC-load-misses,LLC-loads,LLC-prefetch-misses,LLC-prefetches ./ssca2 17
# /usr/bin/time perf stat -e LLC-store-misses,LLC-stores,branch-load-misses,branch-loads,dTLB-load-misses,dTLB-loads ./ssca2 17
# /usr/bin/time perf stat -e dTLB-store-misses,dTLB-stores,iTLB-load-misses,iTLB-loads,node-load-misses,node-loads ./ssca2 17
# /usr/bin/time perf stat -e node-prefetch-misses,node-prefetches,node-store-misses,node-stores ./ssca2 17
/usr/bin/time ./ssca2 17

