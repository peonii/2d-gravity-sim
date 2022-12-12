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

#[macro_export]
macro_rules! input {
    ($ui:ident : $obj:ident @ $range:expr => $name:expr) => {
        $ui.add(Label::new(RichText::new($name)).size(20.0));
        $ui.add(Slider::new(&mut $obj, $range));
    };
    (clamp $ui:ident : $obj:ident @ $range:expr => $name:expr) => {
        $ui.add(Label::new(RichText::new($name).size(20.0)));
        $ui.add(Slider::new(&mut $obj, $range).clamp_to_range(true));
    };
}
