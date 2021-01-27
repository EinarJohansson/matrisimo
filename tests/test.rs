#[cfg(test)]
mod tests {
    #[test]
    fn skapa_matriser() {
        let p: Vec<Vec<u8>> = vec![vec![2, 5, 3], vec![9, 8, 7]];
        let q: Vec<Vec<i32>> = vec![vec![-1, -5, 1], vec![4, 1, 0]]; 
        let u: Vec<Vec<f32>> = vec![vec![1.1, 3.2]];
        let w: Vec<Vec<f64>> = vec![vec![3.0, 4.2]];
    
        let x = matrisimo::Matris::new(&p);
        let y = matrisimo::Matris::new(&q);
        let z = matrisimo::Matris::new(&u);
        let q = matrisimo::Matris::new(&w);
        
        assert_eq!(x.form, (2, 3));
        assert_eq!(y.form, (2, 3));
        assert_eq!(z.form, (1, 2));
        assert_eq!(q.form, (1, 2));
        
        println!("{:?}", x);
        println!("{:?}", y);
        println!("{:?}", z);
        println!("{:?}", q);

/*         y.multiplcera_skalar(&4);
        
        x.multiplcera_skalar(&3); // 4a
        y.subtrahera(&x);
    
        println!("{:?}", y); 

        assert_eq!(2 + 2, 4); */
    }
}
