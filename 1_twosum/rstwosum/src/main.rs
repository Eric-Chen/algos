use std::collections::HashMap;

fn main(){
    let a = [1,2,3,4,5,6,7];
    println!("{:?}", twosum(&a, 8));
    println!("{:?}", twosum(&a, 7));
}

fn twosum(arr :&[i32], target :i32) -> (i32, i32){
    let mut m: HashMap<i32, i32> = HashMap::new();
    for k in 0..arr.len() {
        let v = arr[k];
        let idx_option = m.get(&(target - v));
        match idx_option {
            None => {
                m.insert(v, k as i32);
                continue;
            }
            Some(idx) => {
                return (*idx, k as i32);
            }
        }
    }
    return (-1, -1);
}