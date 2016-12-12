
use point::Point;
use std::f64::consts;


#[derive(Debug, PartialEq)]
pub struct PointCollection {
    points: Vec<Point>,
    size: usize
}

impl PointCollection {
    pub fn new() -> PointCollection {
        PointCollection{
            size: 0usize,
            points: vec![],
        }
    }

    pub fn add_point(&mut self,p: Point) {
        self.points.push(p);
        self.size += 1;
    }

    pub fn get_huelle(&self) -> Vec<usize>{
        let mut huelle = vec![self.get_highest_point()];
        let mut remaining: Vec<usize> = (0..self.size).collect();
        let mut angle = -consts::PI;

        loop {

            let mut smallest_angle = consts::PI;
            let mut next_point = 0usize;
            for i in 0..remaining.len() {
                let curr_angle = self.points[i].get_winkel_from_point(&self.points[*huelle.last().unwrap()]);
                println!("From: {:?}, To: {:?}, Angle: {}, Curr_Angle:{}, smallest_angle: {}",self.points[*huelle.last().unwrap()],self.points[remaining[i]], angle, curr_angle, smallest_angle);
                    if curr_angle < smallest_angle && curr_angle > angle {
                        next_point = i;
                        smallest_angle = curr_angle;
                    }
                }
            if remaining[next_point] == huelle[0]{
                break;
            }
            huelle.push(remaining.remove(next_point));
            angle = smallest_angle;

        }
        huelle

    }

    pub fn get_highest_point(&self) -> usize {
        let mut highest = 0usize;
        for i in 0..self.points.len(){
            if self.points[i].y > self.points[highest].y {
                highest = i;
            }
        }
        highest
    }
}

#[macro_export]
macro_rules! point_collection {
    ($($x:expr ,$y:expr),*) => {
    {
    let mut pc = PointCollection::new();
        $(
            pc.add_point(Point::new($x,$y));
        )*
        pc
    }
    };
}






#[cfg(test)]
mod tests {
    use point::Point;
    use super::PointCollection;

    #[test]
    fn test_add_point(){
        let mut pc = PointCollection::new();
        let p = Point::new(1.0,1.0);

        pc.add_point(p);
        assert_eq!(PointCollection{points: vec![Point::new(1.0,1.0)], size: 1usize},pc)
    }
    #[test]
    fn test_get_highest_point(){
        let mut pc = PointCollection::new();
        pc.add_point(point!(0.0,0.0));
        pc.add_point(point!(0.0,1.0));
        pc.add_point(point!(0.0,2.0));
        pc.add_point(point!(0.0,1.0));

        assert_eq!(2usize, pc.get_highest_point());
        pc.add_point(point!(0.0,2.0));
        assert_eq!(2usize, pc.get_highest_point());


    }

    #[test]
    fn test_pc_macro(){
        let pc = point_collection![1.0, 2.0,
                                   2.3, 4.5,
                                   3.4, 1.5,
                                   -1.0, 3.6,
                                   5.2, 1.3];
        let mut pc2  = PointCollection::new();
        pc2.add_point(point![1.0, 2.0]);
        pc2.add_point(point![2.3, 4.5]);
        pc2.add_point(point![3.4, 1.5]);
        pc2.add_point(point![-1.0, 3.6]);
        pc2.add_point(point![5.2, 1.3]);

        assert_eq!(pc,pc2);
    }

    #[test]
    fn test_get_huelle(){
        let pc = point_collection![1.0, 2.0,
                                   2.3, 4.5,
                                   3.4, 1.5,
                                   -1.0, 3.6,
                                   5.2, 1.3,
                                   2.0, 2.0,
                                   2.1, 2.1];
        assert_eq!(vec![1,3,0,2,4],pc.get_huelle());

    }


}

