extern crate pythagorean_triplet;

#[test]
fn test_triplets_summing_to_initial() {
    assert_eq!(
        pythagorean_triplet::triplets_summing_to(1000).next(), 
        Some((1, 1, 998))
    );
}

#[test]
fn test_triplets_summing_to_last() {
    assert_eq!(
        pythagorean_triplet::triplets_summing_to(1000).last(), 
        Some((333, 333, 334))
    );
}

#[test]
fn test_triplets_summing_to_c_gt_b_gt_a() {
    for (a, b, c) in pythagorean_triplet::triplets_summing_to(1000) {
        assert!(c >= a && c >= b, "{} {} {}", a, b, c);
    }
}

#[test]
fn test_triplets_summing_to_first_ten() {
    let first_ten : Vec<(u32, u32, u32)> = 
        pythagorean_triplet::triplets_summing_to(1000)
            .take(10)
            .collect();
    assert_eq!(
        first_ten,
        vec![
            (1, 1, 998), (1, 2, 997), (1, 3, 996), (2, 2, 996), (1, 4, 995),
            (2, 3, 995), (1, 5, 994), (2, 4, 994), (3, 3, 994), (1, 6, 993)
        ]
    );
}

#[test]
fn test_is_pythagorean_triplet() {
    assert_eq!(pythagorean_triplet::is_pythagorean_triplet(3, 4, 5), true);
}

#[test]
fn test_is_pythagorean_triplet_2() {
    assert_eq!(pythagorean_triplet::is_pythagorean_triplet(9, 40, 41), true);
}

#[test]
fn test_is_pythagorean_triplet_3() {
    assert_eq!(pythagorean_triplet::is_pythagorean_triplet(5, 12, 13), true);
}

#[test]
fn test_pythagorean_triplet_summing_to() {
    for i in pythagorean_triplet::triplets_summing_to(30) {
        println!("{:?}", i);
    }
    assert_eq!(
        pythagorean_triplet::pythagorean_triplet_summing_to(30), Some((5, 12, 13))
    );
}

#[test]
fn test_answer() {
    assert_eq!(pythagorean_triplet::find(), Some(31875000));
}
