#[macro_export]
macro_rules! line_from {
    /*
    ($el:ident $modf:expr => $obj:ident named $name:expr) => {
        Line::new(PlotPoints::from_iter(
            $obj.iter().map(|item| [item.time, item.$el * $modf]),
        ))
        .name($name)
    };
    */
    ($el:ident => $obj:ident named $name:expr) => {
        Line::new(PlotPoints::from_iter(
            $obj.iter().map(|item| [item.time, item.$el]),
        ))
        .name($name)
    };
}
