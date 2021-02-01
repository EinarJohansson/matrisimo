use num::{Num, Zero, One, pow};
use std::ops::{Neg, AddAssign, Mul, MulAssign, SubAssign};

#[derive(Debug, PartialEq)]
pub enum Egenskaper{
    Kvadrat, 
    Rektangel
}

#[derive(Debug, PartialEq)]
pub struct Matris<T: Num> {
    pub matris: Vec<Vec<T>>,
    pub form: (usize, usize),
    pub egenskaper: Vec<Egenskaper>
}

impl<T: Num + Clone> Matris<T>{
    pub fn new(matris: &Vec<Vec<T>>) -> Self {
        let form = Matris::form(&matris);
        let egenskaper = Matris::<T>::egenskaper(&form);

        Self {
            matris: matris.to_vec(),
            form: form,
            egenskaper: egenskaper
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

    fn egenskaper(form: &(usize, usize))-> Vec<Egenskaper> {
        match form {	
            (m, n) if (m==n) => vec![Egenskaper::Kvadrat],	
            _=> vec![Egenskaper::Rektangel]	
        }
    }
}

pub trait Funktioner<T: Num> {
    fn enhets_matris(matris: &Matris<T>) -> Matris<T>
        where T: Zero + One + Copy;
    fn transponera(matris: &Matris<T>) -> Matris<T>
        where T: Zero + Copy;
    fn determinant(matris: &Matris<T>) -> T
        where T: Zero + One + AddAssign + Mul + Copy+ Neg<Output=T>;
    fn adjunkt(matris:  &Matris<T>) -> Matris<T>
        where T: Zero + One + Copy + AddAssign + Mul+ Neg<Output=T>;
}

pub trait Operationer<T: Num> {
    fn addera(&mut self, matris: &Matris<T>)
        where T: AddAssign + Copy;
    fn subtrahera(&mut self, matris: &Matris<T>)
        where T: SubAssign + Copy;
    fn multiplicera_skalar(&mut self, skalar: &T) 
        where T: MulAssign + Copy;
    fn multiplicera_matris(&mut self, matris: &Matris<T>)
        where T: Zero + AddAssign + Mul + Copy;
}

impl<T: Num> Operationer<T> for Matris<T> {
    fn addera(&mut self, matris: &Matris<T>) 
    where T: AddAssign + Copy,{
        for (i, rad) in self.matris.iter_mut().enumerate() {
            for (j, kol) in rad.iter_mut().enumerate() {
                *kol += matris.matris[i][j];
            }
        }
    }
    
    fn subtrahera(&mut self, matris: &Matris<T>)
    where T: SubAssign + Copy,{
        for (i, rad) in self.matris.iter_mut().enumerate() {
            for (j, kol) in rad.iter_mut().enumerate() {
                *kol -= matris.matris[i][j];
            }
        }
    }
    
    fn multiplicera_skalar(&mut self, skalar: &T)
    where T: MulAssign + Copy,{
        for rad in self.matris.iter_mut() {
            for kol in rad.iter_mut() {
                *kol *= *skalar;
            }
        }
    }

    fn multiplicera_matris(&mut self, matris: &Matris<T>)
    where T: Zero + AddAssign +Mul + Copy {
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

impl<T: Num> Funktioner<T> for Matris<T> {
    fn enhets_matris(matris: &Matris<T>) -> Matris<T>
    where T: Zero + One + Copy, {
        assert!(matris.egenskaper.contains(&Egenskaper::Kvadrat));
        let rang = matris.form.0;
        let mut enhets_matris = vec![vec![T::zero(); rang]; rang];
        
        for i in 0..rang {
            for j in 0..rang {
                if i == j {
                    enhets_matris[i][j] = T::one();
                }
            }
        }
        Matris::new(&enhets_matris)
    }

    fn transponera(matris: &Matris<T>) -> Matris<T>
    where T: Zero + Copy, {
        let mut c = vec![vec![T::zero(); matris.form.0]; matris.form.1];
        
        for i in 0..matris.form.0 {
            for j in 0..matris.form.1 {
                c[j][i] = matris.matris[i][j]
            }
        }
        Matris::new(&c)
    }

    // Ta bort rang parameter
    fn determinant(matris:  &Matris<T>) -> T
    where T: Zero + One + Copy + AddAssign + Mul+ Neg<Output=T> {
        assert!(matris.egenskaper.contains(&Egenskaper::Kvadrat));
        let rang = matris.form.0;

        match rang {
            1 => matris.matris[0][0],
            2 => matris.matris[0][0] * matris.matris[1][1] - matris.matris[1][0] * matris.matris[0][1],
            _=> {
                let mut d = T::zero();
                let mut temp = vec![vec![T::zero(); rang-1]; rang-1];

                for x in 0..rang {
                    let mut subi = 0;
                    for i in 1..rang {
                        let mut subj = 0;
                        for j in 0..rang {
                            if j == x {
                                continue;
                            }
                            temp[subi][subj] = matris.matris[i][j];
                            subj += 1;
                        }
                        subi += 1;
                    }
                    let temp = Matris::new(&temp);

                    d += pow(-T::one(), x) * matris.matris[0][x] * Matris::determinant(&temp);
                }
                d
            }
        }
    }

    fn adjunkt(matris: &Matris<T>) -> Matris<T>
    where T: Zero + One + Copy + AddAssign + Mul+ Neg<Output=T> {
        assert!(matris.egenskaper.contains(&Egenskaper::Kvadrat));

        let rang = matris.form.0;

        match rang {
            1 => {
                let adjunkt = vec![vec![T::one()]];
                Matris::new(&adjunkt)
            },
            2 => {
                let adjunkt = vec![vec![matris.matris[1][1], -matris.matris[0][1]], vec![-matris.matris[1][0], matris.matris[0][0]]];
                Matris::new(&adjunkt)
            },
            _ => {
                let mut augmenterad = vec![vec![T::zero(); rang-1]; rang-1];
                let mut adjunkt = vec![vec![T::zero(); rang]; rang];

                for rad_i in 0..rang {
                    for kol_j in 0..rang {

                        let mut i_sub = 0;
                        let mut j_sub = 0; 
                        
                        for rad in 0..rang {
                            for kol in 0..rang {
                                if rad != rad_i && kol != kol_j {
                                    augmenterad[i_sub][j_sub] = matris.matris[rad][kol]; 
                                    j_sub += 1;
    
                                    if j_sub == rang -1 { 
                                        j_sub = 0; 
                                        i_sub += 1; 
                                    }
                                } 
                            }
                        }

                        let sign = match rad_i + kol_j {
                            s if s % 2 == 0 => T::one(),
                            _=> -T::one()
                        };
                        
                        let augmenterad = Matris::new(&augmenterad);
                        adjunkt[kol_j][rad_i] = sign * Matris::determinant(&augmenterad);
                    }
                }
                Matris::new(&adjunkt)
            }
        }
    }
}