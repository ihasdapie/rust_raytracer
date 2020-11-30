use std::ops;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Vec3{
    pub e: [f64; 3],
}


impl Vec3{
    pub fn new(e0:f64, e1:f64, e2:f64) -> Vec3 {
        Vec3{
            e: [e0, e1, e2]
        }
    }

    pub fn r(self) -> f64 {
        self.e[0]
    }
    pub fn g(self) -> f64 {
        self.e[1]
    }
    pub fn b(self) -> f64 {
        self.e[2]
    }
  
    pub fn x(self) -> f64 {
        self.e[0]
    }
    pub fn y(self) -> f64 {
        self.e[1]
    }
    pub fn z(self) -> f64 {
        self.e[2]
    }
    pub fn length(self) -> f64 {
        (self.e[0].powi(2)+
         self.e[1].powi(2)+
         self.e[2].powi(2)).sqrt()
    }

    pub fn unit_vector(v: &Vec3) -> Vec3 {
        // doing by copy which isn't great
        *v / v.length()
    }

}

impl ops::Div<f64> for Vec3{
    type Output = Self;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 { e: [
            self.e[0] / rhs,
            self.e[1] / rhs,
            self.e[2] / rhs,
        ] }
    }
}

impl ops::Add for Vec3{
    type Output= Self;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {e: [
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2]]
        }
    }
}


impl ops::Sub for Vec3{
    type Output= Self;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {e: [
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2]]
        }
    }
}


impl ops::Mul<f64> for Vec3 {
    // multiply vec3 with float, element-wise
    type Output = Self;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs,
                self.e[1] * rhs,
                self.e[2] * rhs,
            ]
        }
    }
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_add (){
        assert_eq!(
            Vec3::new(1f64, 2f64, 3f64) + Vec3::new(3f64, 5f64, 8f64),
            Vec3::new(4f64, 7f64, 11f64))
    }

    #[test]
    fn test_vec_mul (){
        assert_eq!(
            Vec3::new(1f64, 2f64, 3f64) * 8f64,
            Vec3::new(8f64, 16f64, 24f64))
    }

    #[test]
    fn test_vec_div(){
          assert_eq!(Vec3::new(8f64, 16f64, 24f64) / 8f64,
            Vec3::new(1f64, 2f64, 3f64))
    }

}
