fn main() {

    let mut vector: Vec<i32> = vec![1, 2, 33, 2, 4, 1, 3, 5, 9, 11, 7, 99];

    // Selection sorting
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
