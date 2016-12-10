

#[derive(Clone,Debug, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Point {
        Point{
            x: x,
            y: y
        }
    }

    pub fn get_winkel_from_zero(&self) -> f64 {
        self.y.atan2(self.x)
    }

    pub fn get_winkel_from_point(&self, other: &Point) -> f64 {
        (self.y-other.y).atan2(self.x - other.x)
    }

}

#[macro_export]
macro_rules! point {
    ($x:expr ,$y:expr) => {
    Point::new($x,$y)
    };
}

#[cfg(test)]
mod tests{
    use super::Point;
    use std::f64::consts;

    #[test]
    fn test_get_winkel_from_zero() {
        let p = Point::new(1.0,1.0);
        let p2 = Point::new(0.0,0.0);
        let p3 = Point::new(-1.0,-1.0);

        assert_eq!(consts::FRAC_PI_4, p.get_winkel_from_zero());
        assert_eq!(0.0, p2.get_winkel_from_zero());
        assert_eq!(-consts::FRAC_PI_4 * 3.0, p3.get_winkel_from_zero());
    }

    #[test]
    fn test_get_winkel_from_point(){
        let p1 = Point::new(1.0,0.0);
        let p2 = Point::new(2.0,1.0);
        assert_eq!(consts::FRAC_PI_4, p2.get_winkel_from_point(&p1));
        assert_eq!(-consts::FRAC_PI_4 * 3.0, p1.get_winkel_from_point(&p2));
    }

    #[test]
    fn test_macro(){
        assert_eq!(Point::new(0.0,0.0), point!(0.0,0.0))
    }

}