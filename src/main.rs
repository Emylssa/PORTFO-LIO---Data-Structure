fn main() {
    let y = vec![2.0, 4.0, 6.0, 8.0];
    match timewise_regression::regressao_linear(&y) {
        Some((b0, b1)) => println!("Coeficientes: b0 = {}, b1 = {}", b0, b1),
        None => println!("Erro ao calcular regressão linear."),
    }
}
