
use point::Point;


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
        let mut huelle: Vec<usize> = vec![self.get_highest_point()];
        let mut remaining: Vec<usize> = (0..self.size).collect();
        loop {
            let x = self.get_next_in_huelle(*huelle.last().unwrap(), &mut remaining);

            if x == huelle[0] {
                break;
            } else {
                huelle.push(x);
            }

        }
        huelle


    }

    fn get_next_in_huelle(&self, n: usize, rem: &mut Vec<usize>) -> usize {
        let mut winkel = 0f64;
        let mut remove = 0usize;
        for i in 0..rem.len(){
            let curr_winkel = self.points[n].get_winkel_from_point(&self.points[rem[i]]);
            if  curr_winkel < winkel {
                winkel = curr_winkel;
                remove = i;
            }
        }
        rem.remove(remove)

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


}

