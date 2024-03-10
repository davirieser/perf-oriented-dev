
import sys
import time
import json
import subprocess

programs = [
    # Arg 0 = Delannoy Number between 0 and 22.
    { "name": "delannoy", "args": [ "12" ] },
    # Arg 0 = Number of Directories to generate.
    # Arg 1 = Number of Files per Directory.
    # Arg 2 = Minimum File Size.
    # Arg 3 = Maximum File Size.
    # Arg 4 (optional) = RNG Seed.
    { "name": "filegen", "args": [ "5", "25", "1024000", "2048000" ] },
    # No Args
    { "name": "filesearch", "args": [] },
    # No Args
    { "name": "mmul", "args": [] },
    # No Args
    { "name": "nbody", "args": [] },
    # Arg 0 = Problem File to solve.
    { "name": "qap", "args": [ "qap/problems/chr15b.dat" ] },
]

# NOTE: This function was generated by ChatGPT
def parse_key_value_pairs(output):
    key_value_pairs = output.splitlines()
    key_value_map = {}

    for pair in key_value_pairs:
        if ':' in pair:
            key, value = pair.split(':', 1)
            key = key.strip()
            value = float(value.strip())
            key_value_map[key] = value

    return key_value_map

metrics = [ 
    { 
        "name": "Real Time", 
        "fmt-string": "%e", 
    },
    {
        "name": "User Time",
        "fmt-string": "%U",
    },
    {
        "name": "System Time",
        "fmt-string": "%S",
    },
    {
        "name": "Max. Memory",
        "fmt-string": "%M",
    }
]
time_format_string = '\n'.join([f"{metric['name']}:{metric['fmt-string']}" for metric in metrics])

def measure_program(program):
    result = subprocess.run(["/bin/time", f"-f {time_format_string}", f"build/{program['name']}"] + program["args"], capture_output=True)

    success = result.returncode == 0

    program_output = result.stdout.decode("utf-8")
    time_output = result.stderr.decode("utf-8")

    metrics = parse_key_value_pairs(time_output)

    return { "success": success, "metrics": metrics, "output": program_output }

if __name__ == "__main__":
    if (len(sys.argv) > 1):
        repetitions = int(sys.argv[1])
        if (repetitions < 1):
            print("Repetitions has to be greater or equal 1")
            sys.exit(-1)
    else:
        repetitions = 1
    
    results = []
    for program in programs:
        time_stamp = time.strftime("%H:%M:%S %d.%m.%Y", time.localtime())

        print(f"Running Program \"{program['name']}\" {repetitions} times")

        mean = { metric["name"]: None for metric in metrics }
        successful_runs = 0

        for i in range(repetitions): 
            result = measure_program(program)
            print(result)
            if (not result["success"]):
                print(f"> Repetition {i} of Command \"{program['name']}\" did not finish successfully") 
            else:
                for metric_name, metric_value in result["metrics"].items():
                    if (mean[metric_name] == None):
                        mean[metric_name] = metric_value
                    else:
                        mean[metric_name] = (mean[metric_name] * successful_runs + metric_value) / (successful_runs + 1)
                successful_runs += 1

        results.append(program | { "mean": mean, "time_stamp": time_stamp })


    json_dumps = [ json.dumps(result, indent=4, skipkeys=True) for result in results ]
    with open("results.json", 'a+') as f:
        f.writelines(json_dumps)
        f.write('\n')


