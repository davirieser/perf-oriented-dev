#!/bin/bash

# Execute job in the partition "lva" unless you have special requirements.
#SBATCH --partition=lva
# Name your job to be able to identify it later
#SBATCH --job-name TimedMassif 
# Redirect output stream to this file
#SBATCH --output=npb_bt_timed_no_massif.log
# Maximum number of tasks (=processes) to start in total
#SBATCH --ntasks=1
# Maximum number of tasks (=processes) to start per node
#SBATCH --ntasks-per-node=1
# Enforce exclusive node allocation, do not share with other jobs
#SBATCH --exclusive

# /usr/bin/time valgrind --tool=massif ./npb_bt_a
/usr/bin/time ./npb_bt_a

