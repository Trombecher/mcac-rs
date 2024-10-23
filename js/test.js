/**
 * @template T
 *
 * @param previousRight {T[]}
 * @param collect {T[][][]}
 * @param targetLength {number}
 * @param previousLeft {T[]}
 * @param max {number}
 */
export function distribution(
    previousRight,
    collect,
    targetLength,
    previousLeft = [],
    max = previousRight.length
) {
    if(previousLeft.length === targetLength) {
        collect.push([previousLeft, previousRight]);
        return;
    }

    for(let i = 0; i < max; i++) {
        const right = [...previousRight];
        const left = right.splice(i, 1);
        left.push(...previousLeft);
        let l = collect.length;
        distribution(right, collect, targetLength, left, i);
        if(previousLeft.length === previousRight.length) {
            const remove = (l - collect.length) / 2;
            collect.splice(-remove, remove);
        }
    }
}

/**
 * @template T
 *
 * @param s {T[]}
 * @returns {T[][]}
 */
function generatePowerSet(s) {
    if(s.length === 0) return [[]];

    /** @type {T[][]} */
    let powerSet = [];

    /** @type {T[]} */
    const t = new Array(s.length - 1);
    for(let i = 1; i < s.length; i++)
        t[i - 1] = s[i];

    const pt = generatePowerSet(t);
    powerSet.push(...pt);
    for(const element of pt)
        powerSet.push([s[0], ...element])

    return powerSet;
}

const N = 10;

console.log(generatePowerSet((new Array(N)).fill(0).map((_, i) => i)).reduce((acc, set) => acc + `pub static C${set.join("")}:[u8;${set.length}]=[${set.join(",")}];`, ""));

let s = `pub static DIST:[&[(&[u8],&[u8])];${N - 2}]=[`;

for(let n = 3; n <= N; n++) {
    const arr = new Array(n);
    for(let i = 0; i < n; i++) arr[i] = i;

    s += "&[";

    s += generatePowerSet(arr).reduce((acc, set) => {
        if(set.length === 0 || set.length > n / 2) return acc;

        const counterSet = [];

        for(let i = 0; i < n; i++) {
            if(set.indexOf(i) === -1) counterSet.push(i);
        }

        return acc + `(&C${set.join("")},&C${counterSet.join("")}),`;
    }, "")

    s += "],";
}

s += "];";
console.log(s);