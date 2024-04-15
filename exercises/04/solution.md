
## A) Memory profiling

The perturbation for ssca2 is 100% and for npb_bt_a it is 898%.

## B) Measuring CPU counters

##### SSCA2

| Total Events   | Event Name                  | Percentage |
|----------------|-----------------------------|----------|
| 4,300,038,368  | L1-dcache-load-misses:u     | (66.67%) |
| 11,363,216,279 | L1-dcache-loads:u           | (66.67%) |
| 301            | L1-dcache-prefetch-misses:u | (66.67%) |
| 0              | L1-dcache-prefetches:u      | (66.67%) |
| 654,773,039    | L1-dcache-store-misses:u    | (66.67%) |
| 2,654,971,596  | L1-dcache-stores:u          | (66.67%) |
| 634,891        | L1-icache-load-misses:u     | (50.00%) |
| 32,805,521,672 | L1-icache-loads:u           | (66.67%) |
| 295,808,535    | LLC-load-misses:u           | (66.67%) |
| 2,797,138,578  | LLC-loads:u                 | (66.67%) |
| 996,650        | LLC-prefetch-misses:u       | (33.33%) |
| 5,110,828      | LLC-prefetches:u            | (33.33%) |
| 39,780,445     | LLC-store-misses:u          | (66.66%) |
| 1,934,152,430  | LLC-stores:u                | (66.67%) |
| 11,011,225,979 | branch-load-misses:u        | (66.67%) |
| 5,825,195,123  | branch-loads:u              | (66.67%) |
| 1,444,097,732  | dTLB-load-misses:u          | (66.67%) |
| 11,363,234,621 | dTLB-loads:u                | (66.67%) |
| 285,917,149    | dTLB-store-misses:u         | (66.66%) |
| 2,652,661,320  | dTLB-stores:u               | (66.67%) |
| 401,912        | iTLB-load-misses:u          | (66.67%) |
| 34,382,850,822 | iTLB-loads:u                | (66.67%) |
| 1,985          | node-load-misses:u          | (66.67%) |
| 290,274,218    | node-loads:u                | (66.66%) |
| 491            | node-prefetch-misses:u      | (50.00%) |
| 955,146        | node-prefetches:u           | (50.00%) |
| 0              | node-store-misses:u         | (50.00%) |
| 38,173,455     | node-stores:u               | (50.00%) |

##### NPB BT A

| Total Events      | Event Name                  | Percentage |
|-------------------|-----------------------------|----------|
| 29,648,618,021    | L1-dcache-load-misses:u     | (66.67%) |
| 652,910,723,818   | L1-dcache-loads:u           | (66.67%) |
| 1                 | L1-dcache-prefetch-misses:u | (66.67%) |
| 0                 | L1-dcache-prefetches:u      | (66.67%) |
| 10,049,271,475    | L1-dcache-store-misses:u    | (66.67%) |
| 307,427,005,278   | L1-dcache-stores:u          | (66.67%) |
| 208,106,430       | L1-icache-load-misses:u     | (50.00%) |
| 589,174,680,874   | L1-icache-loads:u           | (66.67%) |
| 1,281,619,433     | LLC-load-misses:u           | (66.67%) |
| 2,501,692,070     | LLC-loads:u                 | (66.67%) |
| 2,094,091,054     | LLC-prefetch-misses:u       | (33.33%) |
| 3,325,723,834     | LLC-prefetches:u            | (33.33%) |
| 111,330,506       | LLC-store-misses:u          | (66.67%) |
| 2,065,412,994     | LLC-stores:u                | (66.67%) |
| 7,459,904,561     | branch-load-misses:u        | (66.67%) |
| 7,660,596,440     | branch-loads:u              | (66.67%) |
| 89,104,069        | dTLB-load-misses:u          | (66.67%) |
| 653,017,494,245   | dTLB-loads:u                | (66.67%) |
| 26,009,089        | dTLB-store-misses:u         | (66.67%) |
| 307,426,809,168   | dTLB-stores:u               | (66.67%) |
| 3,726,978         | iTLB-load-misses:u          | (66.67%) |
| 1,597,377,558,730 | iTLB-loads:u                | (66.67%) |
| 5,673             | node-load-misses:u          | (66.67%) |
| 1,281,565,573     | node-loads:u                | (66.67%) |
| 7,941             | node-prefetch-misses:u      | (50.00%) |
| 2,097,695,102     | node-prefetches:u           | (50.00%) |
| 0                 | node-store-misses:u         | (50.00%) |
| 109,681,790       | node-stores:u               | (50.00%) |

I am not sure if perf measured correctly because all the percentages are either 1/2, 1/3 or 2/3 and the percentages seem to be the same for both programs.
I don't think any interpretations of the results would be productive here, since the numbers seem off.
The perturbation is basically not noticable for both programs.

