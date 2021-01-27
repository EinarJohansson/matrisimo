use num::Float;
use num::Num;

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
    fn multiplcera_skalar(&mut self, skalar: &T) where T: std::ops::MulAssign + Copy;
}

impl<T: Num> Operationer<T> for Matris<T> {
    fn addera(&mut self, matris: &Matris<T>) where T: std::ops::AddAssign + Copy, {
        for (i, rad) in self.matris.iter_mut().enumerate() {
            for (j, kol) in rad.iter_mut().enumerate() {
                *kol += matris.matris[i][j];
            }
        }
    }
    
    fn subtrahera(&mut self, matris: &Matris<T>) where T: std::ops::SubAssign + Copy, {
        for (i, rad) in self.matris.iter_mut().enumerate() {
            for (j, kol) in rad.iter_mut().enumerate() {
                *kol -= matris.matris[i][j];
            }
        }
    }
    
    fn multiplcera_skalar(&mut self, skalar: &T) where T: std::ops::MulAssign + Copy, {
        for rad in self.matris.iter_mut() {
            for kol in rad.iter_mut() {
                *kol *= *skalar;
            }
        }
    }
}