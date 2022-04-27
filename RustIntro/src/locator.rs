use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityQueue<T: PartialOrd> {
    fn enqueue(&mut self, ele: T) -> ();
    fn dequeue(&mut self) -> Option<T>;
    fn peek(&self) -> Option<&T>;
}

/**
    An optional definition of a Node struct you may find useful
**/
struct Node<T> {
    priority: i32,
    data: T,
}

/** 
    These traits are implemented for Nodes to make them comparable 
**/
impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority)
    }
}

impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool {
        self.priority == other.priority
    }
}


/** 
    You must implement the above trait for the vector type 
**/
impl<T: PartialOrd> PriorityQueue<T> for Vec<T> {
/**
        This functions pushes a given element onto the queue and
        reorders the queue such that the min heap property holds.
        See the project specifications for more details on how this
        works.
**/
    fn enqueue(&mut self, ele: T) -> () {
        self.push(ele);
        let mut curr = self.len()-1;

        if self.len() > 1 {
        while self[curr] < self[(((curr as i32)-1)/2) as usize] && curr != 0{
            self.swap(curr,(((curr as i32)-1)/2) as usize);
            curr = (((curr as i32)-1)/2) as usize;
        }
        }
    }

/**
        This function removes the root element from the queue and
        reorders the queue such that it maintains the min heap
        property.  See the project specifications for more details.
        You should return the deleted element in the form of an option.
        Return None if the queue was initially empty, Some(T) otherwise.
**/
    fn dequeue(&mut self) -> Option<T> {
       
        if self.len() == 0 {
            return None
        } else if self.len() == 1 {
            return self.pop()
        } else {
            let last = self.len()-1;
            self.swap(0,last);
            let ans = self.remove(last);
            let mut curr = 0;
            let mut min;

         while curr <= self.len()-1{

            let left_tree = 2*curr+1;
            let right_tree = 2*curr+2;
            
            if right_tree <= self.len()-1{
                if self[right_tree] < self[left_tree] {
                    min = right_tree;
                }
                else {
                    min = left_tree;
                }
                if self[curr] > self[min] {
                self.swap(curr,min);
                curr = min;
                }
                else{
                curr = self.len();
            }
            }
            else if left_tree <= self.len()-1{
             let min = left_tree;
             if self[curr] > self[min] {
                self.swap(curr,min);
                curr=min;
             }
             else{
                curr = self.len();
            }
            }
            else{
                curr = self.len();
            }
        }
        return Some(ans);
    }
        }
        

/**
        This function returns the element that would be removed
        if dequeue were called on the queue.  There should be no
        mutations to the queue.  Return the element in the form
        of an option.  Return None if the queue is empty, Some(T)
        otherwise.
**/
    fn peek(&self) -> Option<&T> {
        if self.len() == 0 {
            return None
        } else { 
            return Some(&self[0])
        }
    }
    
}


/**
    You must implement this function that computes the orthogonal
    distance between two coordinates.  Remember, orthogonal distance
    is not like Euclidean distance.  See the specifications for more
    details.
**/
pub fn distance(p1: (i32,i32), p2: (i32,i32)) -> i32 {
    let (x1,y1) = p1;
    let (x2,y2) = p2;

    (x1-x2).abs()+(y1-y2).abs()
}

/**
    You must implement this function that determines which enemy Stark
    should battle and their coordinates.  You are given two hashmaps for
    allies and enemies.  Each maps a name to their current coordinates.
    You can assume that the allies hashmap will always have a name
    called "Stark" included.  Return the name and coordinates of the enemy
    Stark will battle in the form of a 3-tuple.  See the specifications
    for more details on how to choose which enemy.
**/
pub fn target_locator<'a>(allies: &'a HashMap<&String, (i32,i32)>, enemies: &'a HashMap<&String, (i32,i32)>) -> (&'a str,i32,i32) {
    let mut heap = Vec::new();
    let mut tracker = Vec::new();

    for (key, val) in allies.iter() {
        for (key2, val2) in enemies.iter() {
            let n = Node{
                priority: distance(*val,*val2),
                data: [key,key2]
            };
            heap.enqueue(n);
        }
    }
    while !heap.is_empty(){
        {
            let y;
        match heap.peek(){
            Some(n) => y = n,
            None => panic!(),
        };

        if !tracker.contains(y.data[0]) && !tracker.contains(y.data[1]){
            if y.data[0].as_str() == "Stark"{
                return (y.data[1], enemies.get(y.data[1]).unwrap().0,enemies.get(y.data[1]).unwrap().1);
            }
            tracker.push(y.data[0]);
            tracker.push(y.data[1]);
        }        
    }
        heap.dequeue();
    }
    return ("",1,1);
}


