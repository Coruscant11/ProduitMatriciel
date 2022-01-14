use rand::Rng;
use num::complex::Complex;

fn gen_random_complex(rng: &mut rand::rngs::ThreadRng) -> Complex<f32> {
    let random_float: f32 = rng.gen::<f32>() * 50.0;
    let c: Complex<f32> = Complex::new(random_float, random_float*5.2775);
    return c;
}

struct Matrice {
    width: usize,
    height: usize,
    matrix: Vec<Vec<Complex<f32>>>
}

impl Matrice {

    fn new(width: usize, height: usize) -> Matrice {
        Matrice {
            width: width,
            height: height,
            matrix: vec![vec![Complex::<f32>::new(0.0, 0.0); height]; width],
        }
    }

    fn generate_randoms(&mut self, rng: &mut rand::rngs::ThreadRng) -> () {
        for x in 0..self.width {
            for y in 0..self.height {
                self.matrix[x][y] = gen_random_complex(rng)
            }
        }
    }

    fn get_value(&self, x: usize, y: usize) -> Complex<f32> {
        self.matrix[x][y]
    }

    fn set_value(&mut self, x: usize, y: usize, c: Complex<f32>) -> () {
        self.matrix[x][y] = c;
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    const M: usize = 1024;
    const N: usize = 1024;
    const P: usize = 1024;

    let mut a = Matrice::new(N, M); 
    let mut b = Matrice::new(M, P);
    let mut c = Matrice::new(N, P);

    a.generate_randoms(&mut rng);
    b.generate_randoms(&mut rng);
    c.generate_randoms(&mut rng);

    for x in 0..N {
        for y in 0..P {
            for k in 0..M {
                c.set_value(x, y, a.get_value(x, k) * b.get_value(k, y));
            }
        }
    }

    println!("{}", c.get_value(N/2, P/2));
 }