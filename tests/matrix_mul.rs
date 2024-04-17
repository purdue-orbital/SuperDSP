use rustdsp::math::matrix::Matrix;

#[test]
pub fn matrix_mul_2_x_2() {
    let m1 = &[
        &[1, 2][..],
        &[3, 4]
    ];

    let m2 = &[
        &[1, 2][..],
        &[3, 4]
    ];

    let expected = &[
        &[7, 10][..],
        &[15, 22]
    ];

    let real_m1 = Matrix::from_slice(m1);
    let real_m2 = Matrix::from_slice(m2);

    let real_expected = Matrix::from_slice(expected);

    assert_eq!(real_expected, real_m1 * real_m2);
}

#[test]
pub fn matrix_mul_2_x_4() {
    let m1 = &[
        &[1, 2, 3, 4][..],
        &[5, 6, 7, 8]
    ];

    let m2 = &[
        &[9, 10][..],
        &[11, 12],
        &[13, 14],
        &[15, 16],
    ];

    let expected = &[
        &[130, 140][..],
        &[322, 348]
    ];

    let real_m1 = Matrix::from_slice(m1);
    let real_m2 = Matrix::from_slice(m2);

    let real_expected = Matrix::from_slice(expected);

    assert_eq!(real_expected, real_m1 * real_m2);
}

#[test]
pub fn matrix_mul_4_x_2() {
    let m1 = &[
        &[1, 2][..],
        &[3, 4],
        &[5, 6],
        &[7, 8],
    ];

    let m2 = &[
        &[9, 10, 11, 12][..],
        &[13, 14, 15, 16]
    ];


    let expected = &[
        &[35, 38, 41, 44][..],
        &[79, 86, 93, 100],
        &[123, 134, 145, 156],
        &[167, 182, 197, 212],
    ];

    let real_m1 = Matrix::from_slice(m1);
    let real_m2 = Matrix::from_slice(m2);

    let real_expected = Matrix::from_slice(expected);

    assert_eq!(real_expected, real_m1 * real_m2);
}