use num::Num;
use num::Zero;

#[derive(Debug)]
pub struct Matris<T: Num> {
    pub matris: Vec<Vec<T>>,
    pub form: (usize, usize)
}

impl<T: Num + Clone> Matris<T>{
    pub fn new(matris: &Vec<Vec<T>>) -> Self {
        let form = Matris::form(&matris);

        Self {
            matris: matris.to_vec(),
            form: form
        }
    }

    fn form(matris: &Vec<Vec<T>>) -> (usize, usize) {
        assert_eq!(matris.is_empty(), false);
        let rader: usize = matris.len();
        let kolumner: usize = matris[0].len();

        for rad in matris {
            assert_eq!(kolumner, rad.len())
        }

        assert_ne!(kolumner, 0);

        (rader,kolumner)
    }
}

pub trait Operationer<T: Num> {
    fn addera(&mut self, matris: &Matris<T>) where T: std::ops::AddAssign + Copy;
    fn subtrahera(&mut self, matris: &Matris<T>) where T: std::ops::SubAssign + Copy;
    fn multiplicera_skalar(&mut self, skalar: &T) where T: std::ops::MulAssign + Copy;
    fn multiplicera_matris(&mut self, matris: &Matris<T>)
    where T: Zero + std::ops::AddAssign + std::ops::Mul + Copy;
}

impl<T: Num> Operationer<T> for Matris<T> {
    fn addera(&mut self, matris: &Matris<T>) 
    where T: std::ops::AddAssign + Copy,{
        for (i, rad) in self.matris.iter_mut().enumerate() {
            for (j, kol) in rad.iter_mut().enumerate() {
                *kol += matris.matris[i][j];
            }
        }
    }
    
    fn subtrahera(&mut self, matris: &Matris<T>)
    where T: std::ops::SubAssign + Copy,{
        for (i, rad) in self.matris.iter_mut().enumerate() {
            for (j, kol) in rad.iter_mut().enumerate() {
                *kol -= matris.matris[i][j];
            }
        }
    }
    
    fn multiplicera_skalar(&mut self, skalar: &T)
    where T: std::ops::MulAssign + Copy,{
        for rad in self.matris.iter_mut() {
            for kol in rad.iter_mut() {
                *kol *= *skalar;
            }
        }
    }

    fn multiplicera_matris(&mut self, matris: &Matris<T>)
    where T: Zero + std::ops::AddAssign +std::ops::Mul + Copy {
        assert_eq!(self.form.1, matris.form.0);

        let (n, p) = (self.form.0, matris.form.1);
        let mut c = vec![vec![T::zero(); p]; n];

        for i in 0..n {
            for j in 0..p {
                let mut summa: T = T::zero();
                for k in 0..self.form.1 {
                    summa += self.matris[i][k] * matris.matris[k][j];
                }
                c[i][j] = summa;
            }
        }
        *self = Matris::new(&c);
    }
}