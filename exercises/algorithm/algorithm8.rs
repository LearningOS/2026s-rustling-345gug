/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/

#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // 往不为空的队列插入；都空就插入q1
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }    
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // 判断栈整体是否为空
        if self.is_empty() {
            return Err("Stack is empty");
        }
        // 找到非空队列src，空队列dst
        let (src, dst): (&mut Queue<T>, &mut Queue<T>);
        if !self.q1.is_empty() {
            src = &mut self.q1;
            dst = &mut self.q2;
        } else {
            src = &mut self.q2;
            dst = &mut self.q1;
        }
        // src中保留最后一个元素，其余全部移动到dst
        while src.size() > 1 {
            let val = src.dequeue().unwrap();
            dst.enqueue(val);
        }
        // src剩下最后一个，就是栈顶
        src.dequeue()
    }
    pub fn is_empty(&self) -> bool {
        // 两个队列都空，栈才是空
        self.q1.is_empty() && self.q2.is_empty()
    }    
   
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}
