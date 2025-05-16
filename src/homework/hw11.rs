use rand::Rng;
fn gen_random_vector(n: usize) -> Vec<i32> {
    (0..n).map(|_| rand::thread_rng().gen_range(10..100)).collect()
}
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_indices = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indices = (i, i + 1);
        }
    }
    (min_sum, min_indices.0, min_indices.1)
}

fn display_vector(data: &[i32]) {
    let index_line: String = (0..data.len())
        .map(|x| format!("{:>3}.", x))
        .collect();
    let data_line: String = data.iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("indexes: {}", index_line);
    println!("data:   [{}]", data_line);

    let (min_sum, idx1, idx2) = min_adjacent_sum(data);
    let mut arrows = vec!["   "; data.len()];
    arrows[idx1] = r"\__";
    arrows[idx2] = "__/";

    println!("indexes: {}", arrows.join(" "));
    println!("min adjacent sum={}+{}={} at indexes:{},{}", data[idx1], data[idx2], min_sum, idx1, idx2);
}
fn main() {
    let data = gen_random_vector(20);
    display_vector(&data);
}
