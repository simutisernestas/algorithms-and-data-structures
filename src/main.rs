fn main() {
    let mut vector: Vec<i32> = vec![7, 8, 4, 6, 1, 1];

    // selection_sorting(&vector);
    // linear_search(&vector, 5);
    // bubble_sort(&vector);
    // println!("vec: {:?}", merge_sort(&vector));
    println!("{:?}", binary_search(&mut vector, 4));
}

fn binary_search(vector: &mut Vec<i32>, num: i32) -> bool {
    vector.sort();
    
    let middle = vector.len()/2;

    if num == vector[middle] {
        // println!("Position of {:?}: {:?}", num, vector.iter().position(|&r| r == num).unwrap() as i32);
        return true;
    }

    if vector.len() as i32 == 1 {
        return false;
    }

    if num > vector[middle] {
        return binary_search(&mut vector[middle..].to_vec(), num);
    } else {
        return binary_search(&mut vector[..middle].to_vec(), num);
    }
}

// Sort vector by merge method
pub fn merge_sort(vector: &Vec<i32>) -> Vec<i32> {
    let length = vector.len();

    if length == 1 {
        return vector.to_vec();
    }

    let mut left = vector[0..length / 2].to_vec();
    let mut right = vector[length / 2..].to_vec();

    left = merge_sort(&mut left);
    right = merge_sort(&mut right);

    return merge(&left, &right);
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut merged = vec![0; left.len() + right.len()];

    let mut i = 0;
    let mut j = 0;
    let mut index = 0;

    // add until one subarray is deplete
    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged[index] = left[i];
            index += 1;
            i += 1;
        } else {
            merged[index] = right[j];
            index += 1;
            j += 1;
        }
    }

    // add every leftover element from the subarray
    while i < left.len() {
        merged[index] = left[i];
        index += 1;
        i += 1;
    }

    // add every leftover element from the subarray
    while j < right.len() {
        merged[index] = right[j];
        index += 1;
        j += 1;
    }

    return merged;
}

// Sort vector by selection method
fn selection_sorting(vector: &Vec<i32>) {
    let mut vector = vector.clone();

    for i in 0..vector.len() {
        let mut smallest_i = i;

        for j in i..vector.len() {
            if vector[smallest_i] > vector[j] {
                smallest_i = j;
            }
        }

        if vector[i] != vector[smallest_i] {
            vector.swap(i, smallest_i);
        }
    }

    println!("Selection sorted vector: {:?}", vector);
}

// Find element by linear search
fn linear_search(vector: &Vec<i32>, num: i32) {
    for i in 0..vector.len() {
        if &vector[i] != &num {
            println!("Found number: {}, in position: {}", &num, &i);
            return;
        }
    }

    println!("Didn't found number: {}", &num);
    return;
}

// Sort vector by "Bubble sort"
fn bubble_sort(vector: &Vec<i32>) {
    let mut vector = vector.clone();

    let length = vector.len();
    let mut swap = true;

    while swap {
        swap = false;

        for i in 1..length {
            if vector[i - 1] > vector[i] {
                vector.swap(i, i - 1);
                swap = true;
            }
        }
    }

    println!("Bubble sorted vector: {:?}", vector);
}