use iced::Point;

//https://stackoverflow.com/a/12931306
//Maps `value` from interval `[a1;b1]` to the same relative position in `[a2;b2]`
pub fn map_inverval_value(value: f32, from: (f32, f32), to: (f32, f32)) -> f32 {
    let (a1, b1) = from;
    let (a2, b2) = to;
    if a1 == b1 {
        //If from interval has length of zero, than return half of the second interval.
        (b2 + a2) / 2.0
    } else {
        (value - a1) * (b2 - a2) / (b1 - a1) + a2
    }
}

//https://stackoverflow.com/a/6853926
//Finds the distance from a point to an interval (not a line!)
pub fn point_to_interval_distance(point: Point, path_point1: Point, path_point2: Point) -> f32 {
    let x = point.x;
    let y = point.y;
    let x1 = path_point1.x;
    let y1 = path_point1.y;
    let x2 = path_point2.x;
    let y2 = path_point2.y;

    let a = x - x1;
    let b = y - y1;
    let c = x2 - x1;
    let d = y2 - y1;

    let dot = a * c + b * d;
    let len_sq = c * c + d * d;
    let param = if len_sq != 0.0 { dot / len_sq } else { -1.0 };

    let xx;
    let yy;

    if param < 0.0 {
        xx = x1;
        yy = y1;
    } else if param > 1.0 {
        xx = x2;
        yy = y2;
    } else {
        xx = x1 + param * c;
        yy = y1 + param * d;
    }

    let dx = x - xx;
    let dy = y - yy;
    return (dx * dx + dy * dy).sqrt();
}
