
import math
import sys
import time
import json
import typing
import subprocess
import glob
from dataclasses import dataclass

@dataclass
class ComplexJob:
    name: str
    command: str
    arg_matrix: list[list[str]]
    repetitions: int
    env_map: list[dict[str, str]]

    def expand(self): # -> list[SlurmJob]:
        for a in self.arg_matrix:
            for e in self.env_map:
                yield SlurmJob(self.name, self.command, a, self.repetitions, e)

@dataclass
class SlurmJob:
    name: str
    command: str
    args: list[str]
    repetitions: int
    env: dict[str, str]

def is_job_active() -> bool:
    result = subprocess.run(["squeue", "--me"], capture_output = True)
    return result.stdout.decode("utf-8").count('\n') > 1

file_prefix = "runner_output_"

def get_output_files() -> list[str]:
    return glob.glob(f"{file_prefix}*")

def generate_output_file_name(job, number: int):
    return f"{job.name}_{number}"

def get_output_number(file_name: str) -> int:
    return int(file_name.rsplit(file_prefix[-1]), 1)

def run_round(jobs):
    while (is_job_active()):
        time.sleep(5)

    output_files = get_output_files()
    max_number = -1
    for file in output_files:
        number = get_output_number(file)
        if (number > max_number):
            max_number = number

    number = max_number + 1
    
    for j in jobs:
        for i in j.repetitions:
            start_job(j, number)
            number += 1

def start_job(job: SlurmJob, number: int):
    sbatch_header = (
        "#!/bin/bash\n"
        "# SBATCH --partition=lva\n"
        f"# SBATCH --job-name={job.name}\n"
        f"# SBATCH --output={generate_output_file_name(job, number)}\n"
        "# SBATCH --ntasks=1\n"
        "# SBATCH --ntasks-per-node=1\n"
        "# SBATCH --exclusive\n"
    )

def check_is_confident(job):
    return true

if __name__ == "__main__":
    jobs = []
    jobs_to_run = [job for job in jobs if not check_is_confident(job)]
    while (jobs_to_run):
        run_round(jobs_to_run)
        jobs_to_run = [job for job in jobs if not check_is_confident(job)]

