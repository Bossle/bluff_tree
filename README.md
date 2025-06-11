This project will use a linear programming formulation of 2-player zero-sum Nash Equilibria on arbitraty games.

As described by Koller & Meggido and further explored by my thesis, we have, splitting the game tree into two, for each of the players' perspectives,
```
p[node] for each node in the P1 tree signifies the total strategy probability of going to that node
v[node] for each node in the P2 tree signifies the expected value of reaching that node, times the P1 probability of that happening
On a P1 choice node: p[parent] = sum p[child]
On a P1 information node: p[parent] = p[child], for each child
On a P2 choice node: v[parent] <= v[child], for each child
On a P2 information node: v[parent] = sum v[child]
v[leaf2] = sum p[leaf1] * outcome when the game ends at leaf1 and leaf2 * probability that random events lead to leaf1 and leaf2
p[root] = 1
max v[root]
```

This makes it tricky to work with partial trees, for games that are too big. So we can unify them back into one tree:
```
p[node] signifies the total probability of reaching that node
v[node] signifies the expected value if we reach that node
p[parent] * v[parent] = sum p[child] * v[child]
sum p[parent] = sum p[child], for each node
For a random event node, p[parent]/p[child] should follow the game's probabilities
For a P1 choice, sum{information set} p[parent] * v[parent] >= sum{information set} p[child i] * v[child i], for each i
Flip the sign for P2 choice
In the same information set, p[child i] / p[parent] should be constant, for each i
```
We can prove these two formulations are equivalent, and in fact, we can easily move between the two.

This project will have 4 stages:
1. Implementing these formulations on full trees;
2. Applying these formulations on partial trees, that will be the result of randomly exploring the game tree;
3. Adding more direction to the tree exploration, similarly to MCTS, or Stephen Tavener's AiAi UCT. The bandit problem in this case is more complicated, and we may even want to improve on the original: we can take consider a variance in estimated leaves, and see how much that affects the root, and so explore leaves that shoul incur in the largest variance reduction per exploration cycle.
4. Adding neural networks to give estimates to values of cut-off branches
