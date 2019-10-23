**COMP472 - Assignment 1 - October 21, 2019**

**Pierre-André Gagnon - 40067198**

**Problem 1**

a) Breadth-First Search

|Step|State visited|OPEN list|CLOSED list|
|---|---|---|---|
|1||S||
|2|S|A,B,C|S|
|3|A|B,C,D|S,A|
|4|B|C,D,E,F|S,A,B|
|5|C|D,E,F|S,A,B,C|
|6|D|E,F|S,A,B,C,D|
|7|E|F|S,A,B,C,D,E|
|8|F||S,A,B,C,D,E,F|

b) Depth-First Search

|Step|State visited|OPEN list|CLOSED list|
|---|---|---|---|
|1||S||
|2|S|A,B,C|S|
|3|A|D,B,C|S,A|
|4|D|B,F,B,C|S,A,D|
|5|B|E,F,S,B,C|S,A,D,B|
|6|E|B,F,F,S,B,C|S,A,D,B,E|
|7|F|F,S,B,C|S,A,D,B,E,F|

Note that on step 7, a `B` was skipped since it was already in the closed set.

c) Iterative Deepening

*I'm not sure what is meant about the first depth cutoff, so I'll just do it like in the lab.*

Depth 1

|Step|State visited|OPEN list|CLOSED list|
|---|---|---|---|
|1||S(1)||
|2|S||S|

Depth 2

|Step|State visited|OPEN list|CLOSED list|
|---|---|---|---|
|1||S(1)||
|2|S|A(2),B(2),C(2)|S|
|3|A|B(2),C(2)|S,A|
|4|B|C(2)|S,A,B|
|5|C||S,A,B,C|

Depth 3

|Step|State visited|OPEN list|CLOSED list|
|---|---|---|---|
|1||S(1)||
|2|S|A(2),B(2),C(2)|S|
|3|A|D(3),B(2),C(2)|S,A|
|4|D|B(2),C(2)|S,A,D|
|5|B|E(3),F(3),C(2)|S,A,D,B|
|6|E|F(3),C(2)|S,A,D,B,E|
|7|F|C(2)|S,A,D,B,E,F|

d) Uniform Cost Search

|Step|State visited|OPEN list|CLOSED list|
|---|---|---|---|
|1||S(0)||
|2|S|C(3),A(4),B(7)|S|
|3|C|A(4),B(7)|S,C|
|4|A|B(7),D(9)|S,C,A|
|5|B|D(9),E(11),F(13)|S,C,A,B|
|6|D|E(11),F(13)|S,C,A,B,D|
|7|E|F(13)|S,C,A,B,D,E|
|8|F||S,C,A,B,D,E,F|

e) A star search

|Step|State visited|OPEN list|CLOSED list|
|---|---|---|---|
|1||S(4)||
|2|S|C(3+5=8),B(7+4=11),A(4+9=13)|S|
|3|C|B(7+4=11),C(4+9=13)|S,C|
|4|B|A(4+9=13),F(7+6+0=13),D(7+3+4=14),E(7+4+3=14)|S,C,B|
|5|A|F(7+6+0=13),D(7+3+4=14),E(7+4+3=14)|S,C,B,A|
|6|F|D(7+3+4=14),E(7+4+3=14)|S,C,B,A,F|

**Problem 2**

Nodes:
- Max: circle
- Min: square

2.1 Theoretical values:

```
Proceed in reverse order:
G = min(8,6,7) = 6
F = min(10,7) = 7
E = max(G,6) = max(6,6) = 6
D = max(9,F) = max(9,7) = 9
C = min(11,E) = min(11,6) = 6
B = min(D,6,14) = min(9,6,14) = 6
A = max(B,7,C) = max(6,7,6) = 7
```

2.2 Alpha-beta prunning

```
So we will show the value of α and β after visiting each node.
Along the way, we will indicate when α ≥ β and what branch, if any, is pruned.
We will also indicate the missing values along the way.
```

```
Note that we use intendation to indicate the depth of the node.
        9: α = 9 β = ∞
            10: α = 9 β = 10
            7:  α = 9 β = 7  => Prune: nothing to prune
            => F = 7
        F: α = 9 β = ∞
        => D = 9
    D:  α = -∞ β = 9
    6:  α = -∞ β = 6
    14: α = -∞ β = 6
    => B = 6
B: α = 6 β = ∞
7: α = 7 β = ∞
    11: α = 7 β = 11
            8: α = 7 β = 8
            6: α = 7 β = 6  => Prune: node 7
            => G = 6 
        G: α = 6 β = 11
        6: α = 6 β = 11
        => E = 6
    E: α = 7 β = 6          => Prune: nothing to prune
    => C = 6
C: α = 7 β = ∞
=> A = 7
```

2.3 Does it exist a game tree where minimax and alpha-beta prunning algorithms have the potential to produce different moves?

```
No, because the algorithm is specifically designed to produce the exact same
decision by only pruning the branches that cannot influence the final decision.
```

**Problem 3**

```
A: Event of having an accident in the coming year

We are given:
P(good) = 25%
P(average) = 50%
P(bad) = 25%
P(A|good) = 5%
P(A|average) = 15%
P(A|bad) = 25%
```

```
We want to know: P(good|A). We can use Baye's Theorem:

P(good|A) = P(A|good) x P(good)
            -------------------
                    P(A)

P(A) = P(A|good) x P(good) + P(A|average) x P(average) + P(A|bad) x P(bad)
     = 5% x 25% + 15% x 50% + 25% x 25% = 15%

P(good|A) = 5% x 25% = 8.3%
            --------
               15%
```

**Problem 4**

```
HA: Event of having an heart attack

We are given:
P(light) = 30%
P(boost) = 40%
P(no name) = 30%
P(HA|light) = 5%
P(HA|boost) = 20%
P(HA|no name) = 30%
```

```
The question seems to assume that Joe buys his cigarettes at random.
So the probability that he buys one type is the same as the percentage of that
type in the store.

We will use Baye's Theorem so we can start by computing P(HA):
P(HA) = P(HA|light) x P(light) +
        P(HA|boost) x P(boost) +
        P(HA|no name) x P(no name)
      = 5% x 30% + 20% x 40% + 30% x 30% = 18.5%

a) P(light|HA) = P(HA|light) x P(light) = 5% x 30% = 8.11%
                 ----------------------   --------
                         P(HA)             18.5%

b) P(boost|HA) = P(HA|boost) x P(boost) = 20% x 40% = 43.24% 
                 ----------------------   ---------
                         P(HA)             18.5%

c) P(no name|HA) = P(HA|no name) x P(no name) = 30% x 30% = 48.65%
                   --------------------------   --------
                              P(HA)              18.5%
```

**Problem 5**

5.1

```
Remember that H(x,y) = -(x*log₂(x) + y*log₂(y))

CP: Event patient has chess pain
M: Event patient is male
S: Event patient smokes
E: Event patient exercies
HA: Event patient is likely to have a heart attack
```

```
First decision:
H(HA) = H(4/6,2/6) = 0.9183
H(HA|CP = yes) = H(3/3,0/3) = 0
H(HA|CP = no) = H(1/3,2/3) = 0.9183
H(HA|CP) = 3/6 x H(HA|CP = yes) +
           3/6 x H(HA|CP = no)
         = 0.4591
H(HA|M = yes) = H(2/4,2/4) = 1
H(HA|M = no) = H(2/2,0/2) = 0
H(HA|M) = 4/6 x H(HA|M = yes) +
          2/6 x H(HA|M = no)
        = 0.6667
H(HA|S = yes) = H(3/4,1/4) = 0.8113
H(HA|S = no) = H(1/2,1/2) = 1
H(HA|S) = 4/6 x H(HA|S = yes) +
          2/6 x H(HA|S = no)
        = 0.8742
H(HA|E = yes) = H(2/4,2/4) = 1
H(HA|E = no) = H(2/2,0/2) = 0
H(HA|E) = 4/6 x H(HA|E = yes) +
          2/6 x H(HA|E = no)
        = 0.667
The highest information gain will come from the attribute with lowest entropy.
gain(CP) = H(HA) - H(HA|CP) = 0.9183 - 0.4591 = 0.4591
So the root will be CP.
```

```
Second decision:
- Looking above, we see that if CP = yes, we always have HA = yes.
So we have to look at the other branch for more information.

HA2: Event patient has no chess pain and is likely to have a heart attack
H(HA2|M = yes) = H(0/2,2/2) = 0
H(HA2|M = no) = H(1/1,0/1) = 0
H(HA2|M) = 0
H(HA2|S = yes) = H(1/2,1/2) = 1
H(HA2|S = no) = H(0/1,1/1) = 0
H(HA2|S) = 2/3 x H(HA2|S = yes) +
           1/3 x H(HA2|S = no)
         = 0.6667
H(HA2|E = yes) = H(0/2,2/2) = 0
H(HA2|E = no) = H(1/1,0/1) = 0
H(HA2|E) = 0
The highest information gain will come from either M or E.
Take M.
gain(M) = H(HA|CP = no) - H(HA2|M) = 0.9183
```

```
Since the last choice has an entropy of 0, we are done since we can already
conclude if the patient is likely to have an heart attack.

We can construct the decision tree:

     CP
    /  \
Yes/    \No
  /      \
HR        M
         / \
     Yes/   \No
       /     \
  NO HR       HR
```

5.2

```
From this decision tree, we can construct the following decision rules:
- Do you have chess pain?
    - If so, you are likely to have a heart attack.
    - If not, are you male?
        - If so, you are not likely to have a heart attack.
        - If no, you are likely to have a heart attack.
```

**Problem 6**

```
H: Event person is huge
G: Event person is good looking
L: Even person speaks: English, French, Other, or None
T: Even person is threat
```

a)

```
We use same formula as above:
H(T) = H(5/10,5/10) = 1
```

b)

```
Again, same process as above:
H(T|H = yes) = H(2/5, 3/5) = 0.9709
H(T|H = no) = H(3/5, 2/5) = 0.9709
H(T|H) = 5/10 x H(T|H = yes) +
         5/10 x H(T|H = no)
       = 0.9709

H(T|G = yes) = H(4/6,2/6) = 0.9183
H(T|G = no) = H(1/4,3/4) = 0.8113
H(T|G) = 6/10 x H(T|G = yes) +
         4/10 x H(T|G = no)
       = 0.8755

H(T|L = English) = H(3/3,0/3) = 0
H(T|L = French) = H(1/5,4/5) = 0,7219
H(T|L = Other) = H(1/1,0/1) = 0
H(T|L = None) = H(0/1,1/1) = 0
H(T|L) = 3/10 x H(T|L = English) +
         5/10 x H(T|L = French) +
         1/10 x H(T|L = Other) +
         1/10 x H(T|L = None)
       = 5/10 x 0,7219 = 0.3610

The highest information gain will come from the attribute with the lowest
entropy, which is L.
gain(L) = H(T) - H(T|L) = 1 - 0.3610 = 0.6390
```

c)

```
We have seen in the previous question that only when a person speaks French
there is uncertainty about whether the person is a threat so we only have to
explore this case.
So only keep the persons that speaks french.

T2: Event when a person speaks French and is a threat

H(T2|H = yes) = H(1/4,3/4) = 0.8113
H(T2|H = no) = H(0/1,1/1) = 0
H(T2|H) = 4/5 x H(T2|H = yes) +
          1/5 x H(T2|H = no)
        = 0.6085

H(T2|G = yes) = H(1/3,2/3) = 0.9183
H(T2|G = no) = H(0/2,2/2) = 0
H(T2|G) = 3/5 x H(T2|G = yes) +
          2/5 x H(T2|G = no)
        = 0.5510

The highest information gain will come from the attribute with the lowest
entropy, which is G.
gain(G) = H(T|L = French) - H(T2|G) = 0.7219 - 0.5510 = 0.1709
```

```
So we have the following tree:

                     L
             ________|_______
            /       / \      \
    English/ French/   \Other \None
          /       /     \      \
         T       G       T      Not T
                / \
            Yes/   \No
              /     \
             H       Not T
            / \
        Yes/   \No
          /     \
    T: 1/3       No data
Not T: 2/3

Note that we cannot conclude on the last level.
For H = Yes, the data is inconclusive so we listed the probability.
For H = No, there is no data to conclude anything.
```

d)

```
Based on the previous tree, we can conclude:
1. Person is a threat because he/she speaks English.
2. Person is not a threat because he/she speaks French and is not good looking.
3. Person is not a threat because he/she speaks French and is not good looking.
```
