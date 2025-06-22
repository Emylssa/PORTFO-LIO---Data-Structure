use timewise_regression::{regressao_linear, calcular_mse, calcular_r2, prever};

#[test]
fn test_regressao_linear_basico() {
    let y = vec![2.0, 4.0, 6.0, 8.0];
    let (b0, b1) = regressao_linear(&y).unwrap();
    assert!((b0 - 2.0).abs() < 1e-6);
    assert!((b1 - 2.0).abs() < 1e-6);
}

#[test]
fn test_mse_e_r2() {
    let y = vec![1.0, 2.0, 3.0, 4.0];
    let (b0, b1) = regressao_linear(&y).unwrap();
    let mse = calcular_mse(&y, b0, b1);
    let r2 = calcular_r2(&y, b0, b1);
    assert!(mse < 1e-6);
    assert!((r2 - 1.0).abs() < 1e-6);
}

#[test]
fn test_previsao() {
    let (b0, b1) = (1.0, 2.0);
    assert_eq!(prever(b0, b1, 4), 9.0);
}
