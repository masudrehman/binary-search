fn main() {
    let mut a = [1, 10, 33, 56, 70, 330, 422];
    let lenght = a.len();
    let index = binary_search(&mut a, lenght as i32, 56);

    if index != -1 {
        println!("index = {}", index);
    } else {
        print!("element not in list");
    }
    
}

fn binary_search(a: &mut [i32], n: i32, key: i32) -> i32 {
    let mut l = 0;
    let mut h = n - 1;

    while l <= h {
        let  mid = (l + h) / 2;
        if key == a[mid as usize] {
            return mid;
        }
        if key < a[mid as usize] {
            h = mid - 1;
        } else {
            l = mid + 1;
        }
    }
    return -1;
}
