use std::cell::RefCell;
use std::cmp::max;
use std::rc::Rc;
use crate::utils::ParseError;
use serde_json::{Result as SerdeResult, Value};

#[derive(Debug, Clone)]
pub struct Pair {
    left: Rc<RefCell<Node>>,
    right: Rc<RefCell<Node>>,
}

impl Pair {
    fn new(left: Node, right: Node) -> Self {
        let left = Rc::new(RefCell::new(left));
        let right = Rc::new(RefCell::new(right));

        Pair { left, right }
    }
}

#[derive(Debug, Clone)]
pub enum Node {
    Number(Rc<RefCell<usize>>),
    Descent(Pair),
}

impl Node {
    fn from_value(v: &Value) -> Result<Self, ParseError> {
        let node = match v {
            Value::Array(a) => {
                if a.len() != 2 {
                    Err(ParseError::new("Invalid number of children"))?
                } else {
                    let left = Node::from_value(&a[0])?;
                    let right = Node::from_value(&a[1])?;
                    Node::Descent(Pair::new(left, right))
                }
            },
            Value::Number(n) => {
                Node::Number(Rc::new(RefCell::new(n.as_u64().ok_or(ParseError::new("Invalid number"))? as usize)))
            },
            _ => Err(ParseError::new("Invalid value type"))?
        };

        Ok(node)
    }

    fn from_split(v: usize) -> Self {
        let left = Node::Number(Rc::new(RefCell::new(v / 2)));
        let right = Node::Number(Rc::new(RefCell::new((v + 1) / 2)));

        Node::Descent(Pair::new(left, right))
    }
}

#[aoc_generator(day18)]
pub fn input_generator(input: &str) -> Result<Vec<Value>, ParseError> {
    let snails = input
        .lines()
        .filter(|s| *s != "")
        .map(|s| serde_json::from_str(s))
        .collect::<SerdeResult<Vec<_>>>().map_err(|_e| ParseError::new("Cannot parse JSON"))?;
        // .iter()
        // .map(|v| Node::from_value(v))
        //.collect::<Result<Vec<_>, ParseError>>()?;

    Ok(snails)
}

impl Node {
    fn height(&self) -> usize {
        match self {
            Node::Number(_) => 0,
            Node::Descent(p) => {
                max(p.left.borrow().height(), p.right.borrow().height()) + 1
            }
        }
    }

    fn max(&self) -> Option<usize> {
        match self {
            Node::Number(v) => Some(*v.borrow()),
            Node::Descent(p) => {
                max(p.left.borrow().max(), p.right.borrow().max())
            }
        }
    }

    fn split(&self) -> bool {
        if let Node::Descent(p) = self {
            let mut v = 0;
            let mut needs_split = false;

            {
                let left = p.left.borrow();

                if let Node::Number(n) = &*left {
                    if *n.borrow() > 9 {
                        v = *n.borrow();
                        needs_split = true;
                    }
                } else {
                    if left.split() {
                        return true;
                    }
                }
            }

            if needs_split {
                *p.left.borrow_mut() = Node::from_split(v);
                return true;
            }

            {
                let right = p.right.borrow();

                if let Node::Number(n) = &*right {
                    if *n.borrow() > 9 {
                        v = *n.borrow();
                        needs_split = true;
                    }
                } else {
                    if right.split() {
                        return true;
                    }
                }
            }

            if needs_split {
                *p.right.borrow_mut() = Node::from_split(v);
                return true;
            }
        }

        return false;
    }

    fn explode_from_right(&self, add: usize) {
        match self {
            Node::Number(v) => {
                let mut v = v.borrow_mut();
                *v += add;
            },
            Node::Descent(p) => {
                p.right.borrow().explode_from_right(add);
            }
        }
    }

    fn explode_from_left(&self, add: usize) {
        match self {
            Node::Number(v) => {
                let mut v = v.borrow_mut();
                *v += add;
            },
            Node::Descent(p) => {
                p.left.borrow().explode_from_left(add);
            }
        }
    }

    fn explode_internal(&self, lt: Option<&Node>, rt: Option<&Node>, level: usize) -> Result<(bool, bool), ParseError> {
        if let Node::Descent(p) = self {

            if level < 5 {
                let child;
                let all;
                {
                    let left = p.left.borrow();
                    let right = p.right.borrow();
                    let result = left.explode_internal(lt, Some(&*right), level + 1)?;
                    child = result.0;
                    all = result.1;
                }

                if child {
                    *p.left.borrow_mut() = Node::Number(Rc::new(RefCell::new(0)));
                    return Ok((false, true));
                } else if all {
                    return Ok((false, true));
                }

                let child;
                let all;
                {
                    let left = p.left.borrow();
                    let right = p.right.borrow();
                    let result = right.explode_internal(Some(&*left), rt, level + 1)?;
                    child = result.0;
                    all = result.1;
                }
                if child {
                    *p.right.borrow_mut() = Node::Number(Rc::new(RefCell::new(0)));
                }

                if all {
                    return Ok((false, true));
                } else {
                    return Ok((false, false));
                }
            }

            let left = p.left.borrow();
            let right = p.right.borrow();
            match (level, &*left, &*right) {
                (5, Node::Number(l), Node::Number(r)) => {

                    if let Some(lp) = lt {
                        lp.explode_from_right(*l.borrow());
                    }

                    if let Some(rp) = rt {
                        rp.explode_from_left(*r.borrow());
                    }
                    return Ok((true, true));
                },
                _ => Err(ParseError::new("Cannot go deeper than level 5"))?
            }

        }

        Ok((false, false))
    }

    fn explode(&self) -> Result<(), ParseError> {
        if let Node::Descent(p) = self {
            let left = p.left.borrow();
            let right = p.right.borrow();

            let (_, all) = left.explode_internal(None, Some(&*right), 2)?;
            if all {
                return Ok(());
            }

            right.explode_internal(Some(&*left), None, 2)?;
        }

        Ok(())
    }

    fn needs_reduction(&self) -> bool {
        if let Some(max) = self.max() {
            max > 9 || self.height() >= 5
        } else {
            false
        }
    }

    fn add(l: Self, r: Self) -> Result<Self, ParseError> {
        let sum = Node::Descent(Pair::new(l, r));
        while sum.needs_reduction() {
            if sum.height() >= 5 {
                sum.explode()?;
            } else {
                sum.split();
            }
        }

        Ok(sum)
    }

    fn magnitude(&self) -> usize {
        match self {
            Node::Number(v) => *v.borrow(),
            Node::Descent(p) => {
                3 * p.left.borrow().magnitude() + 2 * p.right.borrow().magnitude()
            }
        }
    }

    #[allow(dead_code)]
    fn print(&self) -> String {
        match self {
            Node::Number(n) => format!("{}", *n.borrow()),
            Node::Descent(p) => {
                format!("[{},{}]", p.left.borrow().print(), p.right.borrow().print())
            }
        }
    }
}

#[aoc(day18, part1)]
pub fn solve_part1(input: &Vec<Value>) -> Result<usize, ParseError> {
    let snails = input.iter().map(|i| Node::from_value(i)).collect::<Result<Vec<_>, ParseError>>()?;
    let mut sum = snails[0].clone();

    for i in 1..snails.len() {
        sum = Node::add(sum, snails[i].clone())?;
    }

    Ok(sum.magnitude())
}

#[aoc(day18, part2)]
pub fn solve_part2(input: &Vec<Value>) -> Result<usize, ParseError> {
    let mut result = 0;

    for i in 0..input.len() {
        for j in 0..input.len() {
            if i == j {
                continue;
            }

            let snails = input.iter().map(|i| Node::from_value(i)).collect::<Result<Vec<_>, ParseError>>()?;
            let sum = Node::add(snails[i].clone(), snails[j].clone())?.magnitude();
            result = max(sum, result);
        }
    }

    Ok(result)
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::utils::ParseError;

    fn parse_literal(s: &str) -> Result<Node, ParseError> {
        let value = input_generator(s)?;
        Ok(Node::from_value(&value[0])?.clone())
    }

    #[test]
    fn height_5_works() -> Result<(), ParseError> {
        let snail = parse_literal("[[[[[9,8],1],2],3],4]")?;

        Ok(assert_eq!(5, snail.height()))
    }

    #[test]
    fn height_1_works() -> Result<(), ParseError> {
        let snail = parse_literal("[1,4]")?;

        Ok(assert_eq!(1, snail.height()))
    }

    #[test]
    fn max_1() -> Result<(), ParseError> {
        let snail = parse_literal("[[[[[9,8],1],2],3],4]")?;

        Ok(assert_eq!(Some(9), snail.max()))
    }

    #[test]
    fn max_2() -> Result<(), ParseError> {
        let snail = parse_literal("[[[[0,7],4],[15,[0,13]]],[1,1]]")?;

        Ok(assert_eq!(Some(15), snail.max()))
    }

    #[test]
    fn split_1() -> Result<(), ParseError> {
        let snail = parse_literal("[[[[0,7],4],[15,[0,13]]],[1,1]]")?;

        snail.split();

        let expected = "[[[[0,7],4],[[7,8],[0,13]]],[1,1]]";
        Ok(assert_eq!(expected, &snail.print()))
    }

    #[test]
    fn split_2() -> Result<(), ParseError> {
        let snail = parse_literal("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]")?;

        snail.split();

        let expected = "[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]";
        Ok(assert_eq!(expected, &snail.print()))
    }

    #[test]
    fn explode_1() -> Result<(), ParseError> {
        let snail = parse_literal("[[[[[9,8],1],2],3],4]")?;

        snail.explode()?;

        let expected = "[[[[0,9],2],3],4]";
        Ok(assert_eq!(expected, &snail.print()))
    }

    #[test]
    fn explode_2() -> Result<(), ParseError> {
        let snail = parse_literal("[7,[6,[5,[4,[3,2]]]]]")?;

        snail.explode()?;

        let expected = "[7,[6,[5,[7,0]]]]";
        Ok(assert_eq!(expected, &snail.print()))
    }

    #[test]
    fn explode_3() -> Result<(), ParseError> {
        let snail = parse_literal("[[6,[5,[4,[3,2]]]],1]")?;

        snail.explode()?;

        let expected = "[[6,[5,[7,0]]],3]";
        Ok(assert_eq!(expected, &snail.print()))
    }


    #[test]
    fn explode_4() -> Result<(), ParseError> {
        let snail = parse_literal("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]")?;

        snail.explode()?;

        let expected = "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]";
        Ok(assert_eq!(expected, &snail.print()))
    }


    #[test]
    fn explode_5() -> Result<(), ParseError> {
        let snail = parse_literal("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]")?;

        snail.explode()?;

        let expected = "[[3,[2,[8,0]]],[9,[5,[7,0]]]]";
        Ok(assert_eq!(expected, &snail.print()))
    }

    #[test]
    fn magnitude_1() -> Result<(), ParseError> {
        Ok(assert_eq!(143, parse_literal("[[1,2],[[3,4],5]]")?.magnitude()))
    }

    #[test]
    fn magnitude_2() -> Result<(), ParseError> {
        Ok(assert_eq!(1384, parse_literal("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]")?.magnitude()))
    }

    #[test]
    fn magnitude_3() -> Result<(), ParseError> {
        Ok(assert_eq!(445, parse_literal("[[[[1,1],[2,2]],[3,3]],[4,4]]")?.magnitude()))
    }

    #[test]
    fn magnitude_4() -> Result<(), ParseError> {
        Ok(assert_eq!(791, parse_literal("[[[[3,0],[5,3]],[4,4]],[5,5]]")?.magnitude()))
    }

    #[test]
    fn magnitude_5() -> Result<(), ParseError> {
        Ok(assert_eq!(1137, parse_literal("[[[[5,0],[7,4]],[5,5]],[6,6]]")?.magnitude()))
    }

    #[test]
    fn magnitude_6() -> Result<(), ParseError> {
        Ok(assert_eq!(3488, parse_literal("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]")?.magnitude()))
    }

    fn sample() -> &'static str {
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]"
    }

    fn input() -> Result<Vec<Value>, ParseError> {
        Ok(input_generator(sample())?)
    }

    #[test]
    fn part1_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(4140, solve_part1(&data)?))
    }

    #[test]
    fn part2_sample() -> Result<(), ParseError> {
        let data = input()?;
        Ok(assert_eq!(3993, solve_part2(&data)?))
    }
}
