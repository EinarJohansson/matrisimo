#[cfg(test)]
mod tests {
    #[test]
    fn skapa_matriser() {
        let x = vec![vec![1,2,3], vec![4,5,6]];
        let y = vec![vec![2,4,2,1], vec![1,2,2,1],vec![2,2,2,1], vec![2,2,2,1]];

        let m1 = matrisimo::Matris::new(x); // x invalid nu 
        let m2 = matrisimo::Matris::new(y); // y invalid nu

        println!("{:?}", m1);
        println!("{:?}", m2);

        assert_eq!(m1.form, (2,3));
        assert_eq!(m2.form, (4, 4));

        assert_eq!(m1.egenskaper.unwrap().contains(&matrisimo::Egenskaper::Rektangel), true);
        assert_eq!(m2.egenskaper.unwrap().contains(&matrisimo::Egenskaper::Kvadtrat), true);
    }

    #[test]
    fn addera_matriser() {
        let x = vec![vec![1, 2, 3], vec![4, 5, 6]];
        let y = vec![vec![2, 4, 2], vec![1, 4, 6]];
        let svar = vec![vec![3, 6, 5], vec![5, 9, 12]];

        let mut m1 = matrisimo::Matris::new(x); // x invalid nu 
        let m2 = matrisimo::Matris::new(y);     // y invalid nu

        println!("{:?}", m1);
        println!("{:?}", m2);

        m1.addera(&m2);

        println!("{:?}", m1);

        assert_eq!(m1.matris, svar);
    }

    #[test]
    fn subtrahera_matriser() {
        let x = vec![vec![1, 2], vec![4, 5]];
        let y = vec![vec![2, 4], vec![1, 4]];
        let svar = vec![vec![-1, -2], vec![3, 1]];

        let mut m1 = matrisimo::Matris::new(x); // x invalid nu 
        let m2 = matrisimo::Matris::new(y);     // y invalid nu

        println!("{:?}", m1);
        println!("{:?}", m2);

        m1.subtrahera(&m2);

        println!("{:?}", m1);

        assert_eq!(m1.matris, svar);
    }

    #[test]
    fn skalar_multiplikation() {
        let x = vec![vec![1, 2], vec![4, 5]];
        let svar = vec![vec![10, 20], vec![40, 50]];
        
        let mut m1 = matrisimo::Matris::new(x); // x invalid nu 

        println!("{:?}", m1);

        m1.muiplicera_skalar(10);

        println!("{:?}", m1);

        assert_eq!(m1.matris, svar);
    }

    #[test]
    fn multiplicera_matriser() {
        let a = vec![vec![1, 3], vec![4, -1], vec![-5, 10]];
        let b = vec![vec![2, 1], vec![-8, 6]];
        
        let svar = vec![vec![-22, 19], vec![16, -2], vec![-90, 55]];

        let mut m1 = matrisimo::Matris::new(a); // x invalid nu 
        let m2 = matrisimo::Matris::new(b); // y invalid nu 

        println!("{:?}", m1);
        println!("{:?}", m2);

        m1.multiplicera(&m2);

        println!("{:?}", m1);

        assert_eq!(m1.matris, svar);
    }
}
