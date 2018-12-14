//! Helper methods for modifying shapes.

pub use vecmath::vec2_len;

/// Rotate position some rounds.
#[allow(non_snake_case)]
pub fn rotate2d__pos_rounds(pos: [f64; 2], rounds: f64) -> [f64; 2] {
    use vecmath::row_mat2x3_transform_pos2 as tr;

    let angle = rounds * std::f64::consts::PI * 2.0;
    let cos = angle.cos();
    let sin = angle.sin();
    let mat = [
        [cos, -sin, 0.0],
        [sin, cos, 0.0],
    ];
    tr(mat, pos)
}

/// Displaces in circular pattern around position.
#[allow(non_snake_case)]
pub fn displace_circle2d__pos_amplitude_rounds(
    pos: [f64; 2],
    amplitude: f64,
    rounds: f64,
) -> [f64; 2] {
    let angle = rounds * std::f64::consts::PI * 2.0;
    [
        pos[0] + amplitude * angle.cos(),
        pos[1] + amplitude * angle.sin()
    ]
}
