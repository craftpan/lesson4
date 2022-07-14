#[derive(Debug)]
struct Rectangle{
    length:i32,   
    width:i32,  
}

trait GraphArea{
    fn get_length(&self)->i32;
    fn get_width(&self)->i32;
}

impl GraphArea for Rectangle{
    fn get_length(&self)->i32{
        self.length
    }
    fn get_width(&self)->i32{
        self.width
    }
}

fn print_graph_area(item:impl GraphArea){
    println!("length:{},with:{},area:{}",item.get_length(),item.get_width(),item.get_length()*item.get_width());
}
pub fn print(){
   let rectangle= Rectangle{width:30,length:40};
   print_graph_area(rectangle)
}