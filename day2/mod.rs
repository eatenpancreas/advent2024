use divan::black_box as bb;

mod part_1;
mod part_2;

const INPUT: &str = include_str!("input.txt");

#[test]
fn test_parts() {
    assert_eq!(bb(part_1::attempt_2(bb(INPUT))), 246);
    assert_eq!(bb(part_2::attempt_2(bb(INPUT))), 318);
}

#[divan::bench]
pub fn bench_part_1_attempt_2() {
    bb(part_1::attempt_2(bb(INPUT)));
}

#[divan::bench]
pub fn bench_part_2_attempt_2() {
    bb(part_2::attempt_2(bb(INPUT)));
}
