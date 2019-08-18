use rand::Rng; 
use std::collections::LinkedList;
fn main() 
{
    let mut counter  = 0; 
    let mut rectangle_list:LinkedList<Rectangle> = LinkedList::new();
    while counter < 5
    {
        rectangle_list.push_back(Rectangle
        {
            width:rand::thread_rng().gen_range(10,50), 
            height:rand::thread_rng().gen_range(10,50)
        });
        
        let mut iteration =  rectangle_list.iter();
        let rect_obj = iteration.next();
        let rect = rect_obj.unwrap();

        println!("\nProcessing {}.Rectangle",counter+1);
        println!("The Width of Rectangle is {}\nThe Height of Rectangle is {}",rect.width,rect.height);
        println!("The Area of rectangle is {}",rect.get_area());
        println!("The Circumference of rectangle is {}",rect.get_circumference());

        counter = counter + 1;
        println!("Count of List? ->  {}",rectangle_list.len());
    }
}

struct Rectangle // As you know, we cannot define functions inside of structs.
{
    width:u32, //u32 means UnSigned->Positive Integer (32 bit)
    height:u32
}

impl Rectangle // To define functions we'll use impl(implement) keyword.
{
    fn get_area(&self) -> u32 // Self returns incoming struct's reference
    {
        self.width * self.height 
    }

    fn get_circumference(&self) -> u32
    {
        (self.width + self.height) * 2
    }
}