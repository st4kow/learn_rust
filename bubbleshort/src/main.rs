use rand::Rng;

fn main() {
    let mut vector = gen_rand_vec(-1000, 1000, 20);
    println!("This is the original vector: ");
    println!("{:?}",vector);
    shaker_sort(&mut vector);
    println!("This the vecor after sorting");
    println!("{:?}",vector);
}
fn gen_rand_vec(min: i32, max: i32, size: i32) -> Vec<i32> {
    let mut res: Vec<i32> = Vec::new();
    for _ in 0..size {
        res.push(get_rand(min, max));
    }
    res
}
fn get_rand(min: i32, max: i32) -> i32 {
    rand::rng().random_range(min..=max)
}
fn bubble_sort(data: &mut Vec<i32> ) -> &mut Vec<i32> {
    let size = data.len();
    if size < 2 { return data };
    let mut count = 0;
    while count < size {
        for i in 1..(size-count) {
            if *(data.get(i).unwrap()) <  *(data.get(i-1).unwrap()) {
                data.swap(i, i-1);
            }
        }
        count += 1;
        println!("Iteration: {:?}",data);
    }
    data
}
fn shaker_sort(data: &mut Vec<i32> ) -> &mut Vec<i32> {
    let size = data.len();
    if size < 2 { return data };

    let mut count = 0;
    let mut sorted = false;
    while count < size {
        sorted = true;
        for i in (count+1..(size-count)) {
            if *(data.get(i).unwrap()) <  *(data.get(i-1).unwrap()) {
                data.swap(i, i-1);
                sorted = false;
            }
        }
        println!("Up-shake:   {:?}",data);
        for i in (count..(size-count-1)).rev() {
            //println!("{i}");
            if *(data.get(i).unwrap()) >  *(data.get(i+1).unwrap()) {
                data.swap(i, i+1);
                sorted = false;
            }
        }
        println!("Down-shake: {:?}",data);
        count += 1;
        if sorted { return data; }
    }
    data
}


