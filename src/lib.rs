
#[test]
fn test_function_name() {
    
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
        let mut g = Vec::new();

        for _i in 0 .. 10 {
            g.push("Hello".to_string());
            let c = g.capacity();
            println!("{:?}", c);
        }

        println!("{:?}", g);

        let value = u16::pow(2, 8);
        println!("{:?}", value);


    }
}
