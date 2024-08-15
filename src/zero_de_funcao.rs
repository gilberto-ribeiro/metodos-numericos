pub struct Resultado {
    pub raiz: f64,
    pub iteracoes: u32,
    pub erro: f64,
}

pub fn bisseccao(
    f: fn(f64) -> f64,
    intervalo: (f64, f64),
    tolerancia: f64,
) -> Resultado {
    let (mut a, mut b) = intervalo;
    let mut erro: f64 = (b - a).abs();
    let mut i: u32 = 0;
    let mut c: f64 = 0.0;
    while erro >= tolerancia {
        c = 0.5 * (b + a);
        if f(a) * f(c) > 0.0 {
            a = c;
        } else {
            b = c;
        }
        erro = (b - a).abs();
        i += 1;
    }
    Resultado {
        raiz: c,
        iteracoes: i,
        erro,
    }
}

pub fn regula_falsi(
    f: fn(f64) -> f64,
    intervalo: (f64, f64),
    tolerancia: f64,
) -> Resultado {
    let (mut a, mut b) = intervalo;
    let mut erro: f64 = (b - a).abs();
    let mut i: u32 = 0;
    let mut c: f64 = 0.0;
    while erro >= tolerancia {
        c = b - f(b) * (b - a) / (f(b) - f(a));
        if f(a) * f(c) > 0.0 {
            a = c;
        } else {
            b = c;
        }
        erro = (b - a).abs();
        i += 1;
    }
    Resultado {
        raiz: c,
        iteracoes: i,
        erro,
    }
}
