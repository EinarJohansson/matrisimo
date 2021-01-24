pub type Vector2d = Vec<Vec<i32>>;

#[derive(Debug)]
#[derive(PartialEq)]
pub enum Egenskaper {
    Kvadtrat,
    Rektangel
}

#[derive(Debug)]
pub struct Matris {
    pub matris: Vector2d,
    pub form: (usize, usize),
    pub egenskaper: Option<Vec<Egenskaper>>
}

// Statiska funktioner
impl Matris {
    pub fn new(matris: Vector2d) -> Self {
        let form = Matris::form(matris.as_slice());
        let egenskaper = Matris::egenskaper(&form);

        Self {
            form:       form,
            egenskaper: egenskaper,
            matris:     matris
        }
    }

    fn form(matris: &[Vec<i32>]) -> (usize, usize) {
        assert_eq!(matris.is_empty(), false);
        let rader: usize = matris.len();
        let kolumner: usize = matris[0].len();

        for rad in matris {
            assert_eq!(kolumner, rad.len())
        }

        assert_ne!(kolumner, 0);

        (rader,kolumner)
    }

    fn egenskaper(form: &(usize, usize)) -> Option<Vec<Egenskaper>> {
        match form {
            (m, n) if (m==n) => Some(vec![Egenskaper::Kvadtrat]), // Om rader = kolumner
            _=> Some(vec![Egenskaper::Rektangel])
        }
    }
}

impl Matris {
    /*
    Addera två matriser med varandra
    */
    pub fn addera(&mut self, matris: &Matris) {
        assert_eq!(self.form, matris.form);

        for (i, rad) in self.matris.iter_mut().enumerate() {
            for (j, kol) in rad.iter_mut().enumerate() {
                *kol += matris.matris[i][j];
            }
        }
    }

    /*
    Subtrahera två matriser med varandra
    */
    pub fn subtrahera(&mut self, matris: &Matris) {
        assert_eq!(self.form, matris.form);

        for (i, rad) in self.matris.iter_mut().enumerate() {
            for (j, kol) in rad.iter_mut().enumerate() {
                *kol -= matris.matris[i][j];
            }
        }
    }

    /*
    Multiplicera en matris med en skalär
    */
    pub fn muiplicera_skalar(&mut self, skalar: i32) {
        for rad in self.matris.iter_mut() {
            for kol in rad.iter_mut() {
                *kol *= skalar;
            }
        }
    }

    /*
    Mutiplicera två matriser med varandra
    */
    pub fn multiplicera(&mut self, matris: &Matris) {
        assert_eq!(self.form.1, matris.form.0);

        let (n, p) = (self.form.0, matris.form.1);
        let mut c: Vector2d = vec![vec![0; p]; n];
        
        for i in 0..n {
            for j in 0..p {
                let mut summa = 0;
                for k in 0..self.form.1 {
                    summa += self.matris[i][k] * matris.matris[k][j];
                }
                c[i][j] = summa;
            }
        }

        *self = Matris::new(c);
    }
}
