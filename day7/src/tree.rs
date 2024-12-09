#[derive(Debug)]
pub struct Node {
    value: i64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    middle: Option<Box<Node>>,
}

impl Node {
    pub fn new(value: i64) -> Self {
        Node {
            value,
            left: None,
            right: None,
            middle: None,
        }
    }
    pub fn insert_left(&mut self, value: i64) {
        self.left = Some(Box::new( Node::new(self.value+value)));
    }
    pub fn insert_right(&mut self, value: i64) {
        self.right = Some(Box::new(Node::new(self.value * value)));
    }
    pub fn insert_middle(&mut self, value: i64) {
        let new_value = self.value.to_string() + &value.to_string();
        self.middle = Some(Box::new(Node::new(
            new_value.parse::<i64>().unwrap()
        )));
    }
    pub fn build_tree(&mut self,numbers:Vec<i64>,target:i64,use_middle:bool)->bool{
            
        
        if numbers.is_empty(){
            if self.value==target{
                return true
            }else{
                return false;    
            }
            
        }else{
            if self.value > target{
                return false;
            }
        }
        let number = numbers[0];
        self.insert_left(number);
        self.insert_right(number);
        if use_middle{
            self.insert_middle(number);
        }
        
        
        self.left.as_deref_mut().map_or(false, | l| l.build_tree(numbers[1..].to_vec(), target, use_middle)) ||
        self.right.as_deref_mut().map_or(false, |r| r.build_tree(numbers[1..].to_vec(), target, use_middle)) ||
        self.middle.as_deref_mut().map_or(false, |m| m.build_tree(numbers[1..].to_vec(), target, use_middle))
    }
}