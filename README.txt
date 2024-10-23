# Anvil Simulator

Things associated with each `EnchantmentKind`:

HRN in Runtime <=> Data <=> HRN in Compile-Time

## Algorithm

Let items be the `items` to find the best combination to combine.

If there are two items to be combined:
    Let `left` be the first item and let `second` be the second item.

    `combine(left, right)` yields the step including the cost of the combination.

    Then, the algorithm returns the best of `combine(left, right)` and `combine(right, left)` in form of a branch including this single step.

If there are more items than 2:
    The `items` are distributed into a `left` list and a `right` list.

    For both of the lists the algorithm is called (recursively) to get
        `left_branches` as all the possible branches emerging from the left list and
        `right_branches` as all the possible branches emerging from the right list.

    Each branch has a resulting item, so for each combination of branches of the left and right the algorithm is called



### The distribution function

For n numbers there are n! distributions of length n. Goal is to print all of those.

#### n = 3

[]

[0]
[1]
[2]

[0, 1]
[0, 2]
[1, 2]

[0, 1, 2]

#### n = 4

[]

[0]
[1]
[2]
[3]

[0, 1]
[0, 2]
[0, 3]
[1, 2]
[1, 3]
[2, 3]

[0, 1, 2]
[0, 1, 3]
[0, 2, 3]
[1, 2, 3]

[0, 1, 2, 3]

### Calculating The Power Set

P({}) = {{}}
P(S) = P(S \ {first(S)}) U {t U {first(S)} : t E P(S \ {first(S)})}

---

P({0}) = P({0} \ {first(S)}) U {t U {first(S)} : t E P({0} \ {first({0})})}

P({0}) = P({0} \ {0}) U {t U {0} : t E P({0} \ {0})}

P({0}) = P({}) U {t U {0} : t E P({})}

P({0}) = {{}} U {t U {0} : t E {{}}}

P({0}) = {{}} U {{} U {0}}

P({0}) = {{}} U {{0}}

P({0}) = {{}, {0}}