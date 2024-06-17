
### A) Setup and Basic Execution

##### GCC - No Jump Table

```
100      x fibonacci_naive(30) time:  13.0095 s  --  832040
10000000 x fibonacci_tail(30)  time:  12.8127 s  --  832040
25000000 x fibonacci_iter(30)  time:  11.7029 s  --  832040
```

##### GCC - With Jump Table

```
100      x fibonacci_naive(30) time:  12.9171 s  --  832040
10000000 x fibonacci_tail(30)  time:  12.6853 s  --  832040
25000000 x fibonacci_iter(30)  time:  10.9147 s  --  832040
```

##### Clang - No Jump Table

```
100      x fibonacci_naive(30) time:  14.4256 s  --  832040
10000000 x fibonacci_tail(30)  time:  13.9045 s  --  832040
25000000 x fibonacci_iter(30)  time:  12.8259 s  --  832040
```

##### Clang - With Jump Table

```
100      x fibonacci_naive(30) time:  14.1838 s  --  832040
10000000 x fibonacci_tail(30)  time:  13.5195 s  --  832040
25000000 x fibonacci_iter(30)  time:  12.8535 s  --  832040
```

##### GCC 8 - No Jump Table 

```
100 x fibonacci_naive(30)     time:  13.3761 s  --  832040
10000000 x fibonacci_tail(30) time:  12.3656 s  --  832040
25000000 x fibonacci_iter(30) time:  11.8834 s  --  832040
```

##### GCC 8 - With Jump Table 

```
100 x fibonacci_naive(30)     time:  13.2953 s  --  832040
10000000 x fibonacci_tail(30) time:  12.2686 s  --  832040
25000000 x fibonacci_iter(30) time:  12.0594 s  --  832040
```

### B) Profiling

The tool of choice we used is cachegrind.
We seperated each fibonacci variant into a seperate file and removed the measuring overhead, so we could get more specific results.

Cachegrind measures the amount of operations a program has to do instead of how long the program takes to get more reliable results.
Using `cg_annotate` you can display the results.

###### Naive

```
--------------------------------------------------------------------------------
Ir                       file:function
--------------------------------------------------------------------------------
59,148,312,271 (75.13%)  /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lvm.c:luaV_execute
15,500,936,460 (19.69%)  /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/ldo.c:luaD_precall
 4,079,196,149 ( 5.18%)  /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/ltable.c:luaH_getshortstr
```

###### Tail

```
--------------------------------------------------------------------------------
Ir                      I1mr             ILmr        Dr                      D1mr             DLmr         Dw                     D1mw             DLmw          file:function
--------------------------------------------------------------------------------
67,091,792,728 (63.77%) 357,202 (39.88%) 60 ( 2.73%) 12,560,537,047 (55.15%) 357,148 (33.58%)   8 ( 0.30%) 4,770,536,231 (40.86%)       4 ( 0.00%)   0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lvm.c:luaV_execute
29,450,002,945 (27.99%)       5 ( 0.00%)  5 ( 0.23%)  7,750,000,775 (34.03%)       0            0          5,580,000,558 (47.79%)       0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/ldo.c:luaD_pretailcall
 1,693,777,998 ( 1.61%)      23 ( 0.00%) 15 ( 0.68%)    475,007,958 ( 2.09%)  13,933 ( 1.31%)   0            201,253,533 ( 1.72%)       1 ( 0.00%)   0           ???:_int_free
   970,017,586 ( 0.92%)       8 ( 0.00%)  6 ( 0.27%)    240,003,994 ( 1.05%)       9 ( 0.00%)   0            115,001,189 ( 0.99%)       0            0           ???:malloc
   675,070,814 ( 0.64%)      74 ( 0.01%) 42 ( 1.91%)    175,188,748 ( 0.77%)      18 ( 0.00%)   0            101,262,604 ( 0.87%) 178,915 (49.90%) 344 (27.32%)  ???:_int_malloc
   620,012,862 ( 0.59%)      16 ( 0.00%)  7 ( 0.32%)    160,003,107 ( 0.70%)       4 ( 0.00%)   0            120,002,232 ( 1.03%)       3 ( 0.00%)   2 ( 0.16%)  ???:realloc
   570,000,851 ( 0.54%)      13 ( 0.00%)  9 ( 0.41%)    170,000,262 ( 0.75%)       1 ( 0.00%)   0            120,000,225 ( 1.03%)       1 ( 0.00%)   0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/ldo.c:luaD_precall
   500,007,775 ( 0.48%)       2 ( 0.00%)  2 ( 0.09%)    180,002,799 ( 0.79%)       0            0            100,001,555 ( 0.86%)       0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lmem.c:luaM_malloc_
   460,008,475 ( 0.44%) 178,586 (19.94%)  3 ( 0.14%)    120,002,270 ( 0.53%)  80,603 ( 7.58%)   0             20,000,401 ( 0.17%)       0            0           ???:free
   440,005,896 ( 0.42%)       3 ( 0.00%)  1 ( 0.05%)    140,001,876 ( 0.61%)       0            0            160,002,144 ( 1.37%)       1 ( 0.00%)   1 ( 0.08%)  /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lgc.c:luaC_newobjdt
   420,005,489 ( 0.40%)       7 ( 0.00%)  5 ( 0.23%)    100,001,320 ( 0.44%)       5 ( 0.00%)   3 ( 0.11%)    40,000,762 ( 0.34%)       0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lgc.c:freeobj
   390,000,211 ( 0.37%)       0           0             150,000,082 ( 0.66%)       5 ( 0.00%)   0                      0                0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/ltable.c:luaH_getshortstr
   330,000,046 ( 0.31%)       2 ( 0.00%)  2 ( 0.09%)    110,000,015 ( 0.48%)       0            0             60,000,008 ( 0.51%)       0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lfunc.c:luaF_closeupval
   280,000,073 ( 0.27%)       1 ( 0.00%)  1 ( 0.05%)     30,000,009 ( 0.13%)       0            0             60,000,017 ( 0.51%)       0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lfunc.c:luaF_newLclosure
   260,004,997 ( 0.25%)       1 ( 0.00%)  1 ( 0.05%)     20,000,399 ( 0.09%)       0            0             20,000,399 ( 0.17%)       0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lauxlib.c:l_alloc
   260,004,628 ( 0.25%)       4 ( 0.00%)  1 ( 0.05%)    140,002,492 ( 0.61%)       0            0             60,001,068 ( 0.51%)       0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lmem.c:luaM_free_
   250,000,029 ( 0.24%)       3 ( 0.00%)  3 ( 0.14%)     70,000,009 ( 0.31%)       1 ( 0.00%)   0             80,000,010 ( 0.69%)       0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lfunc.c:luaF_findupval
   243,749,274 ( 0.23%)       3 ( 0.00%)  2 ( 0.09%)     66,964,038 ( 0.29%)  71,898 ( 6.76%)   1 ( 0.04%)    44,821,232 ( 0.38%)       0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lgc.c:sweepgen.isra.4
```

###### Iterative

```
--------------------------------------------------------------------------------
Ir                      I1mr        ILmr        Dr                      D1mr           DLmr         Dw                     D1mw         DLmw          file:function
--------------------------------------------------------------------------------
96,675,004,176 (97.26%) 45 ( 1.73%) 45 ( 2.11%) 17,925,000,795 (95.47%)     6 ( 0.15%)   6 ( 0.23%) 8,550,000,383 (95.80%)   4 ( 0.30%)   0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lvm.c:luaV_execute
 1,425,000,851 ( 1.43%) 13 ( 0.50%)  9 ( 0.42%)    425,000,262 ( 2.26%)     1 ( 0.02%)   0            300,000,225 ( 3.36%)   1 ( 0.08%)   0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/ldo.c:luaD_precall
   750,000,261 ( 0.75%)  0           0             300,000,098 ( 1.60%)     4 ( 0.10%)   0                      0            0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/ltable.c:luaH_getshortstr
   550,000,044 ( 0.55%)  1 ( 0.04%)  1 ( 0.05%)    125,000,010 ( 0.67%)     0            0             75,000,006 ( 0.84%)   0            0           /home/cb76/cb761226/perf-oriented-dev/exercises/12/lua-5.4.6/src/lvm.c:luaV_tointeger
```

##### Result Quality

The result is sufficient to decide optimization decisions on, as it clearly shows all the hot functions, where they are executed and how much time they each take.
Because the benchmarks were done seperately for each fibonnaci variant we also know which functions are important for which variant.
Since this is theoretically only simulated, optimizing branches is not possible but would not really be viable for an interpreter anyway.

### C) Code Understanding

- Major Phases of Lua Interpreter:
  - Command Line Argument Parsing `lua.c` or `luac.c` 
  - Lexing `llex.c`
  - Parsing `lparser.c`
  - Execution `lua_pcallk` in `lua.c`

- LUA_USE_JUMPTABLE is a Compile Flag that is enabled in GCC by default.

  If it is set it imports the `ljumptab.h` on line 1153 of `lvm.c`.

  This import converts the giant switch case in `lvm.c` on line 1183 to the 1896 into a giant label table and `goto` statements by redefining the `vmdispatch` and `vmcase` macros.

### D) Optimization
