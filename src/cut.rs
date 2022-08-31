pub struct LinearCut {
    pub start: Vec2,
    pub end: Vec2,
}

pub struct CurveCut {
    pub start: Vec2,
    pub end: Vec2,
    pub center: Vec2,
    pub clockwise: bool,
}

pub enum Cut {
    Linear(LinearCut),
    Curve(CurveCut),
}

impl LinearCut {
    const linear_regex: Regex = Regex::new(r"X(\d+.\d+)\sY(\d+.\d+)").unwrap();

    fn new(start: Vec2, end: Vec2) -> LinearCut {
        LinearCut {
            start,
            end,
        }
    }

    fn capture(start: Vec2, instruction: &str) -> LinearCut {
        let captures = linear_regex.captures(&line).unwrap();
        let end = Vec2 {
            x: captures.get(1).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
            y: captures.get(2).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
        };

        new(start, end)
    }
}

impl CurveCut {
    const curve_regex: Regex = Regex::new(r"X(\d+.\d+)\sY(\d+.\d+)\sI(\d+.\d+)\sJ(\d+.\d+)").unwrap();

    fn new(start: Vec2, end: Vec2, center: Vec2, clockwise: bool) -> CurveCut {
        CurveCut {
            start,
            end,
            center,
            clockwise,
        }
    }

    fn capture(start: Vec2, instruction: &str, clockwise: bool) -> LinearCut {
        let captures = curve_regex.captures(&line).unwrap();
        let end = Vec2 {
            x: captures.get(1).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
            y: captures.get(2).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
        };
        let center = Vec2 {
            x: captures.get(3).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
            y: captures.get(4).map_or("Panic", |m| m.as_str()).parse::<f32>().unwrap(),
        };

        new(start, end, center, clockwise)
    }
}