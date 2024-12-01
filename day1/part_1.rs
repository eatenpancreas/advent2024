#[inline]
pub fn attempt_2(input: &str) -> i32 {
    let (mut left, mut right) =
        input
            .lines()
            .fold((vec![], vec![]), |(mut left, mut right), line| {
                let mut lr = line.split("   ");
                left.extend(lr.next().and_then(|str| str.parse::<i32>().ok()));
                right.extend(lr.next().and_then(|str| str.parse::<i32>().ok()));
                (left, right)
            });

    left.sort();
    right.sort();

    left.into_iter().enumerate().fold(0, |acc, (i, l)| {
        let r = right[i];
        acc + (l - r).abs()
    })
}

#[inline]
pub fn _attempt_1(input: &str) -> i32 {
    input.split('\n').fold(0, |acc, line| {
        let mut lr = line.split("   ");
        let l = lr
            .next()
            .and_then(|str| str.parse::<i32>().ok())
            .unwrap_or(0);
        let r = lr
            .next()
            .and_then(|str| str.parse::<i32>().ok())
            .unwrap_or(0);
        acc + (l - r).abs()
    })
}

#[inline]
pub fn optim_1(input: &str) -> i32 {
    let mut i = 0;
    let (mut left, mut right) =
        input
            .split_whitespace()
            .fold((vec![], vec![]), |(mut left, mut right), num| {
                (!num.is_empty()).then(|| {
                    let push_to = if i % 2 == 0 { &mut left } else { &mut right };
                    push_to.push(num.parse::<i32>().ok()?);
                    Some(i += 1)
                });
                (left, right)
            });

    left.sort();
    right.sort();

    left.into_iter().enumerate().fold(0, |acc, (i, l)| {
        let r = right[i];
        acc + (l - r).abs()
    })
}

#[inline]
pub fn optim_2(input: &str) -> i32 {
    let mut i = 0;
    let (mut left, mut right) =
        input
            .split_whitespace()
            .fold((vec![], vec![]), |(mut left, mut right), num| {
                (!num.is_empty()).then(|| {
                    let push_to = if i % 2 == 0 { &mut left } else { &mut right };
                    push_to.push(num.parse::<i32>().ok()?);
                    Some(i += 1)
                });
                (left, right)
            });

    left.sort();
    right.sort();

    std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

/// SLOWER!
#[inline]
pub fn optim_3(input: &str) -> i32 {
    let mut left = vec![];
    let mut right = vec![];
    for line in input.lines() {
        let mut items = line.split_whitespace();
        left.push(items.next().unwrap().parse::<i32>().unwrap());
        right.push(items.next().unwrap().parse::<i32>().unwrap());
    }

    left.sort();
    right.sort();

    std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

/// SLOWER!
#[inline]
pub fn optim_4(input: &str) -> i32 {
    let mut i = 0;
    let (mut left, mut right) =
        input
            .split_whitespace()
            .fold((vec![], vec![]), |(mut left, mut right), num| {
                if let Ok(n) = num.parse::<i32>() {
                    let push_to = if i % 2 == 0 { &mut left } else { &mut right };
                    push_to.push(n);
                    i += 1
                }
                (left, right)
            });

    left.sort();
    right.sort();

    std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}

/// FASTEST!
#[inline]
pub fn optim_5(input: &str) -> i32 {
    let mut i = 0;
    let (mut left, mut right) =
        input
            .split(['\n', ' '])
            .fold((vec![], vec![]), |(mut left, mut right), num| {
                (!num.is_empty()).then(|| {
                    let push_to = if i % 2 == 0 { &mut left } else { &mut right };
                    push_to.push(num.parse::<i32>().ok()?);
                    Some(i += 1)
                });
                (left, right)
            });

    left.sort();
    right.sort();

    std::iter::zip(left, right)
        .map(|(l, r)| (l - r).abs())
        .sum()
}
