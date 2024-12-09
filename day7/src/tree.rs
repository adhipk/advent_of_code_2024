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
    pub fn build_tree(&mut self,numbers:Vec<i64>,target:i64){
        if !numbers.is_empty(){
            

            let number = numbers[0];
            self.insert_left(number);
            self.insert_right(number);
            self.insert_middle(number);
            if let Some(l)  = &mut self.left {
                l.build_tree(numbers[1..].to_vec(),target); 
            }
            if let Some(r) = &mut self.right {
                r.build_tree(numbers[1..].to_vec(),target);
            }
            if let Some(m) = &mut self.middle {
                m.build_tree(numbers[1..].to_vec(),target);
            }
        }
    }
    pub fn search_lr(&self, target: i64) -> bool {
        
        if self.value == target && self.right.is_none() && self.left.is_none() {
            return true;
        }
        self.left.as_deref().map_or(false, |l| l.search_lr(target)) ||
        self.right.as_deref().map_or(false, |r| r.search_lr(target))
    }
    pub fn search(&self, target: i64) -> bool {
        if self.value == target && self.left.is_none() && self.right.is_none() && self.middle.is_none() {
            return true;
        }
        self.left.as_deref().map_or(false, |l| l.search(target)) ||
        self.middle.as_deref().map_or(false, |m| m.search(target)) ||
        self.right.as_deref().map_or(false, |r| r.search(target))
    }
}