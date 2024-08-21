use core::f64;

use metodos_numericos::*;

fn main() {
    // let intervalo: (f64, f64) = (300.0, 500.0);
    let tolerancia: f64 = 1e-12;
    let resultado = zero_de_funcao::secante(exercicio_3, (0.0, 1.0), tolerancia);
    println!("Raiz: {}", resultado.raiz);
    println!("Iterações: {}", resultado.iteracoes);
    println!("Erro: {}", resultado.erro);
}

fn exercicio_1(x: f64) -> f64 {
    let temperatura_2: f64 = x;
    let pressao_2: f64 = 1.0;
    let temperatura_1: f64 = 500.0;
    let pressao_1: f64 = 10.0;
    let a: [f64; 5] = [3.259, 1.356e-3, 1.502e-5, -2.374e-8, 1.056e-11];
    (a[0] * temperatura_2.ln()
        + a[1] * temperatura_2
        + 0.5 * a[2] * temperatura_2.powi(2)
        + a[3] * temperatura_2.powi(3) / 3.0
        + 0.25 * a[4] * temperatura_2.powi(4))
        - (a[0] * temperatura_1.ln()
            + a[1] * temperatura_1
            + 0.5 * a[2] * temperatura_1.powi(2)
            + a[3] * temperatura_1.powi(3) / 3.0
            + 0.25 * a[4] * temperatura_1.powi(4))
        - (pressao_2 / pressao_1).ln()
}

fn exercicio_2(x: f64) -> f64 {
    let q: f64 = x;
    let D: f64 = 25.4e-3;
    let v: f64 = (4.0 / 1000.0) * (q / (f64::consts::PI * D * D));
    let epsilon: f64 = 0.0457e-3;
    let delta_p: f64 = -2e5;
    let rho: f64 = 1000.0;
    let mu: f64 = 1e-3;
    let L: f64 = 50.0;
    let Re: f64 = rho * v * D / mu;
    let f: f64 = (-4.0 * (0.27 * epsilon / D + (7.0 / Re).powf(0.9)).log10()).powi(-2);
    let l_wf: f64 = 2.0 * f * L * v * v / D;
    delta_p / rho + l_wf
}

fn exercicio_3(x: f64) -> f64 {
    let z_l: f64 = x;
    let temp: f64 = 100.0;
    let temp_a: f64 = 25.0;
    let temp_w: f64 = 500.0;
    let n: f64 = 3.5;
    (temp - temp_a) / (temp_w - temp_a) - ((n * (1.0 - z_l)).cosh() / (n).cosh())
}