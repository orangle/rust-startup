

fn main() {
    test1();
}

fn test1() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is {}", x);

    // pass to function
    let name = "Lzz".to_string();
    print_my_name(name);
}


fn print_my_name(name: String) {
    println!("{}", name);
}

// https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, thread};

    #[test]
    fn multiple_thread(){
        let mut data = vec![1, 2, 5];
        thread::spawn(move ||{
            // data.push(8);
            println!("thread!");
        });
        data.push(9);
    }

    #[test]
    fn pass_by_reference(){
        // pass by reference
        let map: HashMap<String, String> = HashMap::new();
        print_map(map);

        let mut map2: HashMap<String, String> = HashMap::new();
        map2.insert("lz".to_string(), "name".to_string());
        map2.insert(String::from("age"), String::from("11"));
        print_map_ref(&map2);
    }

    fn print_map(map: HashMap<String, String>) {
        // 怎么打印map
        println!("{:?}", map)
    }

    fn print_map_ref(map: &HashMap<String, String>) {
        println!("{:?}",map)
    }


    #[test]
    fn test_owner() {
        assert_eq!(1, 1);
    }

    #[test]
    fn test_str() {
        let mut s = String::from("hello world");
        s.push_str(", boy");
        println!("{}", s);

        {
            let ss = String::from("girls");
            println!("{}", ss);
        }
        // 这里已经出了作用域，被销毁了
        //println!("{}", ss);
    }

    #[test]
    fn test_move() {
        let s1 = String::from("hello");
        // let s2 = s1;
        println!("{}, world", s1);
    }

    #[test]
    fn test_clone() {
        let s1 = String::from("hello");
        let _s2 = s1.clone();
        println!("{}, world", s1);
    }

    fn tasks_ownership(some_string: String) {
        println!("{}", some_string);
    }

    fn makes_copy(some_integer: i32){
        println!("{}", some_integer);
    }

    #[test]
    fn test_ownership(){
        let s = String::from("hello");
        tasks_ownership(s);
        // println!("{}", s);

        let x  =5;
        makes_copy(x);
    }
}

