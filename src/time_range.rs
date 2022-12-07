use crate::runner::Symulacja;

pub fn calculate_time_range(sym: &Symulacja) -> f64 {
    ((sym.starting_velocity * sym.starting_velocity + sym.starting_y * 2. * sym.starting_y_accel)
        .sqrt()
        + sym.starting_velocity)
        / sym.starting_y_accel
}
