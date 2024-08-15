use numerical_methods::*;

const R: f64 = 8.314;

fn main() {
    let intervalo: (f64, f64) = (300.0, 500.0);
    let tolerancia: f64 = 1e-10;
    let metodos = [zero_de_funcao::bisseccao, zero_de_funcao::regula_falsi];
    for metodo in metodos {
        let resultado = metodo(exercicio_1, intervalo, tolerancia);
        println!("Raiz: {}", resultado.raiz);
        println!("Iterações: {}", resultado.iteracoes);
        println!("Erro: {}", resultado.erro);
    }
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
