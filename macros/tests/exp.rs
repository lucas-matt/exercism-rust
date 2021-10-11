use std::convert::identity;
macro_rules! simple {
    () => {
        1
    }
}

macro_rules! identity {
    ($i:expr) => {
        $i
    }
}

macro_rules! multi {
    ($($a:expr), *) => {
        {
            0
            $(+$a)*
        }
    }
}

macro_rules! keyword {
    ($a:expr => $b:expr) => {
        $a + $b
    }
}



#[test]
fn test_simple() {
    println!("{}", simple!());
}

#[test]
fn test_ident() {
    println!("{}", identity!(1));
    println!("{}", identity!("hello"));
}

#[test]
fn test_multi() {
    println!("{}", multi!(1, 2, 3))
}

#[test]
fn test_keyword() {
    println!("{}", keyword!(1 => 2))
}