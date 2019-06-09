fn main() {
    let mut vector: Vec<i32> = vec![1, 2, 33, 2, 4, 1, 3, 5, 9, 11, 7, 99];

    // selection_sorting(&vector);
    // linear_search(&vector, 5);
    bubble_sort(&vector);
}

// Sort vector by selection
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
            let temporary = vector[i];

            vector[i] = vector[smallest_i];

            vector[smallest_i] = temporary;
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
    let swap = true;

    while swap {
        let mut swap_count = 0;

        for i in 1..length {
            if vector[i - 1] > vector[i] {
                let temp = vector[i];

                vector[i] = vector[i - 1];

                vector[i - 1] = temp;

                swap_count += 1;
            }
        }

        if swap_count == 0 {
            break;
        }
    }

    println!("Bubble sorted vector: {:?}", vector);
}
