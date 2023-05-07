mod theory;
mod experiment;

fn main() {
    println!("Hello, world!");
    experiment::theory_fixed_n();
    experiment::theory_fixed_p();
    experiment::double_spending_simulator();
}
