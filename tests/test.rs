#[cfg(test)]
mod tests {
    #[test]
    fn skapa_matriser() {
        let p: Vec<Vec<u8>> = vec![vec![2, 5, 3], vec![9, 8, 7]];
        let q: Vec<Vec<i32>> = vec![vec![-1, -5, 1], vec![4, 1, 0]]; 
        let a: Vec<Vec<f32>> = vec![vec![1.1, 3.2]];
        let b: Vec<Vec<f64>> = vec![vec![3.0, 4.2]];
    
        let p = matrisimo::Matris::new(&p);
        let q = matrisimo::Matris::new(&q);
        let a = matrisimo::Matris::new(&a);
        let b = matrisimo::Matris::new(&b);
        
        assert_eq!(p.form, (2, 3));
        assert_eq!(q.form, (2, 3));
        assert_eq!(a.form, (1, 2));
        assert_eq!(b.form, (1, 2));
    }
    #[test]
    fn addera_matriser() {
        use matrisimo::Operationer;

        let p: Vec<Vec<i32>> = vec![vec![2, 5, 3], vec![9, 8, 7]];
        let q: Vec<Vec<i32>> = vec![vec![-1, -5, 1], vec![4, 1, 0]]; 
        let pq_svar: Vec<Vec<i32>> = vec![vec![1, 0, 4], vec![13, 9, 7]];

        let mut p = matrisimo::Matris::new(&p);
        let q = matrisimo::Matris::new(&q);
        
        println!("P är lika med: {:?}", p);
        println!("Q är lika med: {:?}", q);
        p.addera(&q);
        println!("P+Q är lika med: {:?}", p);

        assert_eq!(p.matris, pq_svar);
    }

    #[test]
    fn subtrahera_matriser() {
        use matrisimo::Operationer;

        let p: Vec<Vec<i32>> = vec![vec![2, 5, 3], vec![9, 8, 7]];
        let q: Vec<Vec<i32>> = vec![vec![-1, -5, 1], vec![4, 1, 0]]; 
        let pq_svar: Vec<Vec<i32>> = vec![vec![3, 10, 2], vec![5, 7, 7]];

        let mut p = matrisimo::Matris::new(&p);
        let q = matrisimo::Matris::new(&q);
        
        println!("P är lika med: {:?}", p);
        println!("Q är lika med: {:?}", q);
        p.subtrahera(&q);
        println!("P-Q är lika med: {:?}", p);

        assert_eq!(p.matris, pq_svar);
    }

    #[test]
    fn multiplicera_matris_skalar() {
        use matrisimo::Operationer;

        let p: Vec<Vec<i32>> = vec![vec![2, 5, 3], vec![9, 8, 7]];
        const Q:i32 = 34;
        let pq_svar: Vec<Vec<i32>> = vec![vec![68, 170, 102], vec![306, 272, 238]];

        let mut p = matrisimo::Matris::new(&p);

        println!("P är lika med: {:?}", p);
        println!("Q är lika med: {:?}", Q);
        p.multiplicera_skalar(&Q);
        println!("P*Q är lika med: {:?}", p);

        assert_eq!(p.matris, pq_svar);
    }

    #[test]
    fn multiplicera_matriser() {
        use matrisimo::Operationer;
        
        let p: Vec<Vec<f64>> = vec![vec![2.0, 5.0], vec![2.0, 2.0]];
        let q: Vec<Vec<f64>> = vec![vec![1.0, 2.0, 1.0], vec![1.0, 1.0, 0.0]]; 
        let pq_svar: Vec<Vec<f64>> = vec![vec![7.0, 9.0, 2.0], vec![4.0, 6.0, 2.0]];

        let a: Vec<Vec<i32>> = vec![vec![2, 5], vec![9, 8]];
        let b: Vec<Vec<i32>> = vec![vec![-1, -5, 1], vec![4, 1, 0]]; 
        let ab_svar: Vec<Vec<i32>> = vec![vec![18, -5, 2], vec![23, -37, 9]];
        
        let mut p = matrisimo::Matris::new(&p);
        let q = matrisimo::Matris::new(&q);

        let mut a= matrisimo::Matris::new(&a);
        let b = matrisimo::Matris::new(&b);

        println!("P är lika med: {:?}", p);
        println!("A är lika med: {:?}", a);

        p.multiplicera_matris(&q);
        a.multiplicera_matris(&b);

        println!("P är nu lika med: {:?}", p);
        println!("A är nu lika med: {:?}", a);

        assert_eq!(p.matris, pq_svar);
        assert_eq!(a.matris, ab_svar);
    }

    #[test]
    fn transponera_matris() {
        use matrisimo::Funktioner;
        
        let p: Vec<Vec<i32>> = vec![vec![1, 2], vec![3,4], vec![5, 6]]; 
        let p_transponerad: Vec<Vec<i32>> = vec![vec![1, 3, 5], vec![2, 4, 6]]; 

        let p = matrisimo::Matris::new(&p);

        println!("P: {:?}", p);
        let transponant = matrisimo::Matris::transponera(&p);

        println!("P transponerad: {:?}", p);

        assert_eq!(transponant.matris, p_transponerad);
    }

    #[test]
    fn enhets_matris() {
        use matrisimo::Funktioner;
        
        let p: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3,4,1], vec![5, 6,2]];
        let p = matrisimo::Matris::new(&p);
        let svar = vec![vec![1, 0, 0], vec![0, 1, 0], vec![0, 0, 1]];
        println!("P: {:?}", p);
        
        let p_enhets_matris = matrisimo::Matris::enhets_matris(&p);

        println!("P enhetsmatris: {:?}", p_enhets_matris);

        assert_eq!(p_enhets_matris.matris, svar);
    }

    #[test]
    fn determinant() { 
        use matrisimo::Funktioner;

        let p: Vec<Vec<i32>> = vec![
            vec![1, 3, 5, 9],
            vec![1, 3, 1, 7],
            vec![4, 3, 9, 7],
            vec![5, 2, 0, 9]
        ];

        let svar = -376;

        let p = matrisimo::Matris::new(&p);

        let determinant = matrisimo::Matris::determinant(&p);

        println!("deterimnanten är lika med {}", determinant);
        println!("{:?}", p);
        
        assert_eq!(determinant, svar);
    }

    #[test]
    fn adjunkt() {
        use matrisimo::Funktioner;

        let p: Vec<Vec<i32>> = vec![
            vec![-3, 2, -5],
            vec![-1, 0, -2],
            vec![3, -4, 1]
        ];

        let svar = vec![
            vec![-8, 18, -4],
            vec![-5, 12, -1],
            vec![4, -6, 2]
        ];

        let p = matrisimo::Matris::new(&p);
        let svar = matrisimo::Matris::new(&svar);

        let p = matrisimo::Matris::adjunkt(&p);
        println!("{:?}", p);

        assert_eq!(p.matris, svar.matris);
    }

    #[test]
    fn invertera() {
        use matrisimo::Funktioner;

        let p: Vec<Vec<f64>> = vec![
            vec![1.0,   2.0,    3.0],
            vec![4.0,   5.0,    6.0],
            vec![7.0,   2.0,    9.0]
        ];

        let svar: Vec<Vec<f64>> = vec![
            vec![-11.0/12.0,    1.0/3.0,    1.0/12.0],
            vec![-1.0/6.0,      1.0/3.0,    -1.0/6.0],
            vec![3.0/4.0,       -1.0/3.0,   1.0/12.0]
        ];

        let p = matrisimo::Matris::new(&p);

        let invert = matrisimo::Matris::invertera(&p);

        assert_eq!(invert.matris, svar);
    }
}