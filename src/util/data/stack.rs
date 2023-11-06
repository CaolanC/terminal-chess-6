pub struct Stack<T> {
    data: Vec<T>,
    length: usize,
}

impl<T> Stack<T> {
    pub fn new() -> Self {
        Stack {
            data: Vec::new(),
            length: 0,
        }
    }

    pub fn push(&mut self, value: T) {
        self.data.push(value);
        self.length += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.length == 0 {
            return None;
        }
        self.length -= 1;
        self.data.pop()
    }

    pub fn top(&self) -> Option<&T> {
        if self.length == 0 {
            return None;
        }
        self.data.last()
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialisation() {
        let stack: Stack<i32> = Stack::new();
        assert_eq!(stack.len(), 0);
        assert!(stack.is_empty());
    }

    #[test]
    fn write() {
        let mut stack = Stack::new();
        stack.push(1);
        assert_eq!(stack.len(), 1);
        assert!(!stack.is_empty());
    }

    #[test]
    fn read() {
        let mut stack = Stack::new();
        stack.push(1);
        stack.push(2);

        assert_eq!(stack.top(), Some(&2));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.len(), 1);

        assert_eq!(stack.top(), Some(&1));
        assert_eq!(stack.pop(), Some(1));
        assert!(stack.is_empty())
    }
}
