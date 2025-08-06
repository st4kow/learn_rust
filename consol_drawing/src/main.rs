use colorz::{Colorize, xterm};
use::array2d::Array2D;

fn main() {
    let mut map = create_new_map(20, 20);
    //draw_line(2.0, 5.0, 0.0, 10.0, &mut map);
    draw_line(10.0, 10.0, 20.0, 20.0, &mut map);
    draw_section(3.0, 18.0, 10.0, 13.0, &mut map);
    draw_section(3.0, 13.0, 10.0, 18.0, &mut map);
    draw_circle(5.0, 6.0, 4.0, &mut map);
    print_map(&map);
}
fn create_new_map(width: usize, height: usize) -> Array2D<u8>{
    Array2D::filled_with(0u8, width, height)
}
fn print_map(map: &Array2D<u8>) {
    for row in map.as_rows().iter() {
        for pixel in row.iter() {
            if *pixel != 0 {
                print!("{}", (*pixel).to_string().red());
            } else {
                print!("{pixel}");
            }
        }
        println!(""); //Rowend
    }
}
fn draw_line(x1: f64, y1: f64, x2: f64, y2: f64, map: &mut Array2D<u8>) {
    let slope = (y2-y1) / (x2-x1);
    // y-y1 = m(x-x1) -> y = m(x-x1)+y1 -> Draw pixel, where this equals
    let y_min = 0;
    let y_max = map.rows_iter().count()-1;
    for x in 0..map.columns_iter().count(){
        let y_target = ((slope*(x as f64 - x1)) + y1).round() as isize;
        let y_target = if y_target  > -1 { y_target as usize } else { continue; };
        if y_target >= y_min && y_target <= y_max {
            map.set(y_target, x, 1).unwrap();
        }
    }

    // x = (y-y1)/m + x1
    let inv_slope = 1.0 / slope;
    let x_min = 0;
    let x_max = map.columns_iter().count() - 1;
    for y in 0..map.rows_iter().count() {
        let x_target = ((y as f64 - y1) * inv_slope + x1).round() as isize;
        let x_target = if x_target > -1 { x_target as usize } else { continue; };
        if x_target >= x_min && x_target <= x_max {
            map.set(y, x_target, 1).unwrap();
        }
    }
}

fn draw_section(x1: f64, y1: f64, x2: f64, y2: f64, map: &mut Array2D<u8>) {
    let slope = (y2-y1) / (x2-x1);
    // y-y1 = m(x-x1) -> y = m(x-x1)+y1 -> Draw pixel, where this equals

    let y_min = 0;
    let y_max = map.rows_iter().count()-1;
    let x_min: usize = 0;
    let x_max: usize = map.columns_iter().count()-1;

    let x_draw_min: usize = (x1.min(x2) as usize).clamp(x_min, x_max);
    let x_draw_max: usize = (x1.max(x2) as usize).clamp(x_min, x_max);
    for x in x_draw_min..=x_draw_max {
        let y_target = ((slope*(x as f64 - x1)) + y1).round() as isize;
        if y_target >= y_min && y_target <= y_max as isize {
            map.set(y_target as usize, x, 1).unwrap();
        }
    }

    // x = (y-y1)/m + x1
    let inv_slope = 1.0 / slope;
    let y_draw_min: usize = (y1.min(y2) as usize).clamp(y_min as usize, y_max);
    let y_draw_max: usize = (y1.max(y2) as usize).clamp(y_min as usize, y_max);
    for y in y_draw_min..=y_draw_max {
        let x_target = ((y as f64 - y1) * inv_slope + x1).round() as isize;
        if x_target >= x_min as isize && x_target <= x_max as isize {
            map.set(y, x_target as usize, 1).unwrap();
        }
    }
}

fn draw_circle(h: f64, k: f64, r: f64, map: &mut Array2D<u8>) {
    // (x-h)^2 + (y-k)^2 = r^2
    // y =  sqrt( r^2 - (x-h)^2 ) + k
    let y_min = 0;
    let y_max = map.rows_iter().count()-1;
    for x in 0..map.columns_iter().count(){
        let root = (r.powi(2) - ((x as f64 - h).powi(2))).sqrt();
        if root.is_nan() { continue };
        let y_target_1 = (root + k).round() as isize;
        let y_target_2 = (root * (-1.0) + k).round() as isize;
        if y_target_1 >= y_min && y_target_1 <= y_max as isize {
            map.set(y_target_1 as usize, x, 1).unwrap();
        }
        if y_target_2 >= y_min && y_target_2 <= y_max as isize {
            map.set(y_target_2 as usize, x, 1).unwrap();
        }
    }

    //x = sqrt( (r^2) - (y-k)^2 ) + h
    let x_min = 0;
    let x_max = map.columns_iter().count()-1;
    for y in 0..map.rows_iter().count() {
        let root = (r.powi(2) - ((y as f64 - k).powi(2))).sqrt();
        if root.is_nan() { continue };
        let x_target_1 = (root + h).round() as isize;
        let x_target_2 = (root*(-1.0) + h).round() as isize;
        if x_target_1 >= x_min && x_target_1 <= x_max as isize {
            map.set(y, x_target_1 as usize, 1).unwrap();
        }
        if x_target_2 >= x_min && x_target_2 <= x_max as isize {
            map.set(y, x_target_2 as usize, 1).unwrap();
        }
    }
}