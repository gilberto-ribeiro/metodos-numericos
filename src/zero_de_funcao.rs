pub struct Resultado {
    pub raiz: f64,
    pub iteracoes: usize,
    pub erro: f64,
}

pub fn bisseccao(f: fn(f64) -> f64, intervalo: (f64, f64), tolerancia: f64) -> Resultado {
    let (mut a, mut b) = intervalo;
    let mut erro: f64 = (b - a).abs();
    let mut i: usize = 0;
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

pub fn regula_falsi(f: fn(f64) -> f64, intervalo: (f64, f64), tolerancia: f64) -> Resultado {
    let (mut a, mut b) = intervalo;
    let mut erro: f64 = (b - a).abs();
    let mut i: usize = 0;
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

pub fn newton_raphson(
    f: fn(f64) -> f64,
    df: fn(f64) -> f64,
    chute: f64,
    tolerancia: f64,
) -> Resultado {
    let mut x_old: f64 = chute;
    let mut x_new: f64 = 0.0;
    let mut erro: f64 = 1.0;
    let mut i: usize = 0;
    while erro >= tolerancia {
        x_new = x_old - f(x_old) / df(x_old);
        erro = (x_new - x_old).abs();
        x_old = x_new;
        i += 1;
    }
    Resultado {
        raiz: x_new,
        iteracoes: i,
        erro,
    }
}

pub fn secante(f: fn(f64) -> f64, chutes: (f64, f64), tolerancia: f64) -> Resultado {
    let (mut x_older, mut x_old) = chutes;
    let mut x_new: f64 = 0.0;
    let mut erro: f64 = 1.0;
    let mut i: usize = 0;
    while erro >= tolerancia {
        let f_old: f64 = f(x_old);
        let f_older: f64 = f(x_older);
        x_new = (x_older * f_old - x_old * f_older) / (f_old - f_older);
        erro = (x_new - x_old).abs();
        x_older = x_old;
        x_old = x_new;
        i += 1;
    }
    Resultado {
        raiz: x_new,
        iteracoes: i,
        erro,
    }
}
