use rayon::prelude::*;

fn main() {
    let numeros: Vec<u32> = (1..=1_000_000).collect();

    let soma: u64 = numeros.par_iter()
        .map(|&n| (n as u64).pow(2))
        .sum();

    println!("Soma dos quadrados: {}", soma);
}