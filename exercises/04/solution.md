
## A) Memory profiling

For scca2 the peak allocating functions are:
- genScaleData with a peak of 4.0 MiB
- computeGraph with a peak of 4.0 MiB
- betweennessCentrality with a peak of 4.0 MiB

For npb_bt_a the peak allocating functions is:
- _IO_file_doallocate with 1.0 KiB.

The perturbation for ssca2 is ~100% and for npb_bt_a it is 898%.

## B) Measuring CPU counters

##### SSCA2

| Percentage     | Event Name                  |
|----------------|-----------------------------|
| 37.8417365508% | L1-dcache-load-misses:u     |
| ???%           | L1-dcache-prefetch-misses:u |
| 24.6621485513% | L1-dcache-store-misses:u    |
| 0.0019353175%  | L1-icache-load-misses:u     |
| 10.5753979201% | LLC-load-misses:u           |
| 19.5007540852% | LLC-prefetch-misses:u       |
| 2.0567378446%  | LLC-store-misses:u          |
| 189%           | branch-load-misses:u        |
| 12.70850933%   | dTLB-load-misses:u          |
| 10.778501833%  | dTLB-store-misses:u         |
| 0.00116893157% | iTLB-load-misses:u          |
| 0.00068383613% | node-load-misses:u          |
| 0.05140575367% | node-prefetch-misses:u      |
| 0%             | node-store-misses:u         |

| Total Events   | Event Name                  |
|----------------|-----------------------------|
| 4,300,038,368  | L1-dcache-load-misses:u     |
| 11,363,216,279 | L1-dcache-loads:u           |
| 301            | L1-dcache-prefetch-misses:u |
| 0              | L1-dcache-prefetches:u      |
| 654,773,039    | L1-dcache-store-misses:u    |
| 2,654,971,596  | L1-dcache-stores:u          |
| 634,891        | L1-icache-load-misses:u     |
| 32,805,521,672 | L1-icache-loads:u           |
| 295,808,535    | LLC-load-misses:u           |
| 2,797,138,578  | LLC-loads:u                 |
| 996,650        | LLC-prefetch-misses:u       |
| 5,110,828      | LLC-prefetches:u            |
| 39,780,445     | LLC-store-misses:u          |
| 1,934,152,430  | LLC-stores:u                |
| 11,011,225,979 | branch-load-misses:u        |
| 5,825,195,123  | branch-loads:u              |
| 1,444,097,732  | dTLB-load-misses:u          |
| 11,363,234,621 | dTLB-loads:u                |
| 285,917,149    | dTLB-store-misses:u         |
| 2,652,661,320  | dTLB-stores:u               |
| 401,912        | iTLB-load-misses:u          |
| 34,382,850,822 | iTLB-loads:u                |
| 1,985          | node-load-misses:u          |
| 290,274,218    | node-loads:u                |
| 491            | node-prefetch-misses:u      |
| 955,146        | node-prefetches:u           |
| 0              | node-store-misses:u         |
| 38,173,455     | node-stores:u               |

##### NPB BT A

| Percentage     | Event Name                  |
|----------------|-----------------------------|
| 4.54099112473% | L1-dcache-load-misses:u     |
| ???%           | L1-dcache-prefetch-misses:u |
| 3.26883172346% | L1-dcache-store-misses:u    |
| 0.0035321685%  | L1-icache-load-misses:u     |
| 51.2301033516% | LLC-load-misses:u           |
| 62.966474624%  | LLC-prefetch-misses:u       |
| 5.3902297663%  | LLC-store-misses:u          |
| 97.3802055679% | branch-load-misses:u        |
| 0.01364497495% | dTLB-load-misses:u          |
| 0.00846025402% | dTLB-store-misses:u         |
| 0.00023331854% | iTLB-load-misses:u          |
| 0.00044266170% | node-load-misses:u          |
| 0.00037855835% | node-prefetch-misses:u      |
| 0%             | node-store-misses:u         |

| Total Events      | Event Name                  |
|-------------------|-----------------------------|
| 29,648,618,021    | L1-dcache-load-misses:u     |
| 652,910,723,818   | L1-dcache-loads:u           |
| 1                 | L1-dcache-prefetch-misses:u |
| 0                 | L1-dcache-prefetches:u      |
| 10,049,271,475    | L1-dcache-store-misses:u    |
| 307,427,005,278   | L1-dcache-stores:u          |
| 208,106,430       | L1-icache-load-misses:u     |
| 589,174,680,874   | L1-icache-loads:u           |
| 1,281,619,433     | LLC-load-misses:u           |
| 2,501,692,070     | LLC-loads:u                 |
| 2,094,091,054     | LLC-prefetch-misses:u       |
| 3,325,723,834     | LLC-prefetches:u            |
| 111,330,506       | LLC-store-misses:u          |
| 2,065,412,994     | LLC-stores:u                |
| 7,459,904,561     | branch-load-misses:u        |
| 7,660,596,440     | branch-loads:u              |
| 89,104,069        | dTLB-load-misses:u          |
| 653,017,494,245   | dTLB-loads:u                |
| 26,009,089        | dTLB-store-misses:u         |
| 307,426,809,168   | dTLB-stores:u               |
| 3,726,978         | iTLB-load-misses:u          |
| 1,597,377,558,730 | iTLB-loads:u                |
| 5,673             | node-load-misses:u          |
| 1,281,565,573     | node-loads:u                |
| 7,941             | node-prefetch-misses:u      |
| 2,097,695,102     | node-prefetches:u           |
| 0                 | node-store-misses:u         |
| 109,681,790       | node-stores:u               |

#### Conclusion

Both Programs seem to have problems with Branch Prediction.
ssca2 seems to have problems with Layer 1 Cache while npb_bt_a seems to have problems with Layer 3 cache.
The perturbation is basically not noticable for both programs.

