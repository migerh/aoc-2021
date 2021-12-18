import { readFileSync } from 'fs';

function input() {
    let raw = readFileSync(process.argv[2], 'utf8');
    return raw.split('\n')
        .filter(s => s != "")
        .map(s => JSON.parse(s));
}

function get_height(node, level) {
    if (node.left !== undefined) {
        const left = node.left;
        const right = node.right;

        return Math.max(get_height(left, level + 1), get_height(right, level + 1))
    }

    return level;
}

function max_value(node) {
    if (node.left !== undefined) {
        return Math.max(max_value(node.left), max_value(node.right));
    }

    return node.value;
}

function needs_reduction(node) {
    return get_height(node, 0) > 4 || max_value(node) > 9;
}

class Node {
    constructor(n) {
        if (Array.isArray(n)) {
            this.hasChildren = true;
            this.left = n[0].hasChildren ? n[0] : new Node(n[0]);
            this.right = n[1].hasChildren ? n[1] : new Node(n[1]);
        } else {
            this.hasChildren = false;
            this.value = n;
        }
    }

    explodeFromRight(v) {
        let n = this;
        while(n.hasChildren) {
            n = n.right;
        }

        n.value += v;
    }

    explodeFromLeft(v) {
        let n = this;
        while(n.hasChildren) {
            n = n.left;
        }

        n.value += v;
    }

    split() {
        if (this.hasChildren) {
            throw new Error("Cannot split node with children");
        }

        this.hasChildren = true;
        this.left = new Node(Math.floor(this.value / 2));
        this.right = new Node(Math.ceil(this.value / 2));
        this.value = undefined;
    }

    plain() {
        if (!this.hasChildren) {
            return this.value;
        }

        let left = this.left.plain();
        let right = this.right.plain();

        return `[${left},${right}]`;
    }

    magnitude() {
        if (this.hasChildren) {
            let left = this.left.magnitude();
            let right = this.right.magnitude();
            return 3 * left + 2 * right;
        }

        return this.value;
    }
}

function explode(node) {
    let queue = [[node, null, null, null, '', 1]];

    while (queue.length > 0) {
        let [q, lt, rt, previous, dir, level] = queue.pop();

        if(q.hasChildren && level < 5) {
            queue.push([q.right, q.left, rt, q, 'right', level + 1]);
            queue.push([q.left, lt, q.right, q, 'left', level + 1]);
            continue;
        }

        if(q.hasChildren && level === 5) {
            if (q.left.hasChildren || q.right.hasChildren) {
                throw new Error('Cannot nest deeper than 5 levels!');
            }

            let left = q.left.value;
            let right = q.right.value;

            if (lt !== null) {
                lt.explodeFromRight(left);
            }

            if (rt !== null) {
                rt.explodeFromLeft(right);
            }
            previous[dir] = new Node(0);
            break;
        }
    }

    return node;
}

function split(node) {
    let queue = [node];

    while (queue.length > 0) {
        let q = queue.pop();

        if(q.hasChildren) {
            queue.push(q.right);
            queue.push(q.left);
            continue;
        }

        if(q.value > 9) {
            q.split();
            break;
        }
    }

    return node;
}

function reduce(a) {
    while (needs_reduction(a)) {
        if (get_height(a, 0) > 4) {
            a = explode(a);
            continue;
        }

        a = split(a);
    }

    return a;
}

function add(a, b) {
    return reduce(new Node([a, b]));
}


function task1() {
    const snails = input().map(s => new Node(s));

    let sum = snails[0];
    for (let i = 1; i < snails.length; ++i) {
        sum = add(sum, snails[i]);
    }

    console.log('Task 1 result', sum.magnitude());
}

task1();

function task2() {
    let snails = input().map(s => new Node(s));
    let max = 0;

    for (let i = 0; i < snails.length; ++i) {
        for (let j = 0; j < snails.length; ++j) {
            if (i == j) {
                continue;
            }

            let snails = input().map(s => new Node(s));
            let sum = add(snails[i], snails[j]).magnitude();
            max = Math.max(max, sum);
        }
    }

    console.log('Task 2 result', max);
}
task2();


function testExplode() {
    const explodes = [
        [[[[[9,8],1],2],3],4],
        [7,[6,[5,[4,[3,2]]]]],
        [[6,[5,[4,[3,2]]]],1],
        [[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]],
        [[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]],
    ].map(e => new Node(e));

    const expected = [
        '[[[[0,9],2],3],4]',
        '[7,[6,[5,[7,0]]]]',
        '[[6,[5,[7,0]]],3]',
        '[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]',
        '[[3,[2,[8,0]]],[9,[5,[7,0]]]]',
    ];

    for (let i = 0; i < 5; ++i) {
        let e = explode(explodes[i]).plain();
        if (e !== expected[i]) {
            console.error(`Explode #${i+1} failed. Expected "${expected[i]}" but got "${e}"`);
        } else {
            console.error(`Explode #${i+1} success.`);
        }
    }
}

function testAdd() {
    const a = new Node([[[[4,3],4],4],[7,[[8,4],9]]]);
    const b = new Node([1,1]);

    const expected = '[[[[0,7],4],[[7,8],[6,0]]],[8,1]]';

    let e = add(a, b).plain();
    if (e !== expected) {
        console.error(`Add failed. Expected "${expected}" but got "${e}"`);
    } else {
        console.error(`Add success.`);
    }
}

function testMagnitude() {
    const input = [[[1,2],[[3,4],5]],
        [[[[0,7],4],[[7,8],[6,0]]],[8,1]],
        [[[[1,1],[2,2]],[3,3]],[4,4]],
        [[[[3,0],[5,3]],[4,4]],[5,5]],
        [[[[5,0],[7,4]],[5,5]],[6,6]],
        [[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]],
    ].map(i => new Node(i));

    const expected = [143, 1384, 445, 791, 1137, 3488];

    for (let i = 0; i < input.length; ++i) {
        let e = input[i].magnitude();
        if (e !== expected[i]) {
            console.error(`Magnitude #${i+1} failed. Expected "${expected[i]}" but got "${e}"`);
        } else {
            console.error(`Magnitude #${i+1} success.`);
        }
    }
}

// testExplode();
// testAdd();
// testMagnitude();