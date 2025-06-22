pub fn regressao_linear(y: &[f64]) -> Option<(f64, f64)> {
    let n = y.len();
    if n == 0 {
        return None;
    }

    let x: Vec<f64> = (0..n).map(|v| v as f64).collect();

    let media_x = x.iter().sum::<f64>() / n as f64;
    let media_y = y.iter().sum::<f64>() / n as f64;

    let numerador: f64 = x.iter().zip(y.iter())
        .map(|(xi, yi)| (xi - media_x) * (yi - media_y))
        .sum();

    let denominador: f64 = x.iter().map(|xi| (xi - media_x).powi(2)).sum();

    if denominador == 0.0 {
        return None;
    }

    let b1 = numerador / denominador;
    let b0 = media_y - b1 * media_x;

    Some((b0, b1))
}

pub fn calcular_mse(y: &[f64], b0: f64, b1: f64) -> f64 {
    let n = y.len() as f64;
    let mse: f64 = y.iter()
        .enumerate()
        .map(|(i, yi)| {
            let xi = i as f64;
            let y_pred = b0 + b1 * xi;
            (yi - y_pred).powi(2)
        })
        .sum();
    mse / n
}

pub fn calcular_r2(y: &[f64], b0: f64, b1: f64) -> f64 {
    let media_y = y.iter().sum::<f64>() / y.len() as f64;

    let ss_total: f64 = y.iter().map(|yi| (yi - media_y).powi(2)).sum();
    let ss_res: f64 = y.iter()
        .enumerate()
        .map(|(i, yi)| {
            let y_pred = b0 + b1 * i as f64;
            (yi - y_pred).powi(2)
        })
        .sum();

    1.0 - (ss_res / ss_total)
}

pub fn prever(b0: f64, b1: f64, proximo_periodo: usize) -> f64 {
    b0 + b1 * proximo_periodo as f64
}
