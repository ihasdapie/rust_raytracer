mod vec3;
use vec3::Vec3;
mod ray;
use ray::Ray;

fn color(r: &Ray) -> Vec3 {
    // a function to assign color given a position on the 
    // canva
    let unit_direction:Vec3 = Vec3::unit_vector(&r.direction());
    let t:f64 = 0.5*(unit_direction.y()+1.0);
    return Vec3::new(1.0,1.0, 1.0) * (1.0-t) + Vec3::new(0.5, 0.7, 1.0) * t;
}



fn main(){

    // define window dims
    
    let lower_left_corner: Vec3 = Vec3::new(-2.0, -1.0,  -1.0);
    let horizontal: Vec3 = Vec3::new(4.0, -1.0,  -1.0);
    let vertical: Vec3 = Vec3::new(0.0, 2.0,  0.0);
    let origin: Vec3 = Vec3::new(0.0, 0.0,  0.0);


    let w: i32 = 200;
    let h:i32 = 100;
    let max_value:i32 = 255;

    println!("P3\n{} {}\n{}", w, h, max_value);




    for j in (0..h).rev() {
        for i in 0..w {
            let u:f64 = i as f64 / w as f64;
            let v:f64 = j as f64 / h as f64;
            let r: Ray = Ray::ray(origin, lower_left_corner + horizontal*u + vertical*v);
            let col: Vec3 = color(&r);

            let ir:i32 = (255.99*col.r()) as i32;
            let ig:i32 = (255.99*col.g()) as i32;
            let ib:i32 = (255.99*col.b()) as i32;
            println!("{} {} {}", ir, ig, ib);
            
        }
    }



    let v1 = Vec3::new(1f64, 2f64, 6f64);
    let v2 = Vec3::new(2f64, 6f64, 8f64);

    let v3 = v1 + v2;
    // println!("{:?} + {:?} = {:?}", v1, v2 ,v3);

}
