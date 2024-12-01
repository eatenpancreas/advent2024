use divan::black_box as bb;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[test]
fn test_parts() {
    // PART 1: SUCCESS!
    assert_eq!(bb(part_1::attempt_2(bb(INPUT))), 2057374);
    assert_eq!(bb(part_1::optim_1(bb(INPUT))), 2057374);
    assert_eq!(bb(part_1::optim_2(bb(INPUT))), 2057374);
    assert_eq!(bb(part_1::optim_3(bb(INPUT))), 2057374);
    assert_eq!(bb(part_1::optim_4(bb(INPUT))), 2057374);
    assert_eq!(bb(part_1::optim_5(bb(INPUT))), 2057374);
    // PART 2: SUCCESS IN 1 GO!
    assert_eq!(bb(part_2::attempt_1(bb(INPUT))), 23177084);
    assert_eq!(bb(part_2::optim_1(bb(INPUT))), 23177084);
}

// PART 1:

#[divan::bench]
pub fn bench_part_1_attempt_2() {
    bb(part_1::attempt_2(bb(INPUT)));
}

#[divan::bench]
pub fn bench_part_1_optim_1() {
    bb(part_1::optim_1(bb(INPUT)));
}

#[divan::bench]
pub fn bench_part_1_optim_2() {
    bb(part_1::optim_2(bb(INPUT)));
}

#[divan::bench]
pub fn bench_part_1_optim_3() {
    bb(part_1::optim_3(bb(INPUT)));
}

#[divan::bench]
pub fn bench_part_1_optim_4() {
    bb(part_1::optim_4(bb(INPUT)));
}

#[divan::bench]
pub fn bench_part_1_optim_5() {
    bb(part_1::optim_5(bb(INPUT)));
}

// PART 2:

#[divan::bench]
pub fn bench_part_2_attempt_1() {
    bb(part_2::attempt_1(bb(INPUT)));
}
#[divan::bench]
pub fn bench_part_2_optim_1() {
    bb(part_2::optim_1(bb(INPUT)));
}
