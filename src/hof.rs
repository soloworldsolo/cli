
pub fn hof() {
    let mut sum = 0;
    let max = 500;

    let total:i32 = (0 ..).map(|num| num * num)
        .take_while(|&num| num <=max)
        .sum();

    println!("{}" , total);

    let arr = vec![String::from("apple") , String::from("orange") ,String::from("apricot")];
    let mut values:Vec<String> = Vec::new();

     values = arr.iter().map(|a| a.to_uppercase())
         .filter(|a| a.ends_with("E"))
       .collect();
    println!("{:?}" ,values);

        let per = person {
        name : String::from("solomon")
    };

    let dog =Dog {
        name : String::from("jimmy") ,
        owner : &per
    };

    println!("{:?}",dog)


}
#[derive(Debug)]
struct person {
    name : String
}
#[derive(Debug)]
struct Dog<'l> {
    name:String,
    owner: &'l person
}


impl Dog<'_> {
    fn get_name(&mut self) -> &String {
        let mut a = &mut self.name;
        a.push_str(&self.owner.name);
        return  a;
    }
}