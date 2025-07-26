#[macro_export]
macro_rules! my_vec {
    ($elem:expr; $count:expr) => {
        {
            let count = $count;
            let mut tmp_vec = Vec::new();

            for _ in 0..count  {
                tmp_vec.push($elem.clone());
            }
            tmp_vec

        }
    };

    ($($x:expr),*) => {
        {
            let mut my_vec = Vec::new();
            $(
                my_vec.push($x);
            )*
            my_vec
        }
    };


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro() {
        let mut y = my_vec![];
        y.push(1);
        println!("{:?}", y);

        let x = my_vec![1, 2];
        println!("{:?}", x);

        let z = my_vec!(5;3);
        println!("{:?}", z);
    }
}
