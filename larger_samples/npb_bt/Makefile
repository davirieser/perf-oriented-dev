
build_03:
	mkdir -p build
	cd build && \
		cmake .. -G Ninja -DCMAKE_BUILD_TYPE=Release -DCMAKE_CXX_FLAGS=-pg -DCMAKE_EXE_LINKER_FLAGS=-pg -DCMAKE_SHARED_LINKER_FLAGS=-pg && ninja

03_sizes?=s w a b c
03_progs:=$(foreach size,$(03_sizes),npb_bt_$(size))

run_03: build_03
	cd build && $(foreach prog,$(03_progs),./$(prog);mv gmon.out $(prog).gprof;)

analyse_03: run_03 
	cd build && $(foreach prog,$(03_progs),gprof $(prog) $(prog).gprof -b >> $(prog).txt;)

lcc: build_03
	sbatch job.sh

.PHONY: clean
clean:
	rm -rf build

