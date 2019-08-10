pub struct SimpleLinkedList<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

pub struct Node<T> {
    data: T,
    next: Link<T>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn len(&self) -> usize {
        let mut length = 0;
        let mut current = &self.head;
        while current.is_some() {
            length += 1;
            current = current.as_ref().map(|node| &node.next).unwrap();
        }

        length
    }

    pub fn push(&mut self, _element: T) {
        let new_node = Box::new(Node {
            data: _element,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut reversed = SimpleLinkedList::new();
        let mut current = &self.head;
        while let Some(ref node) = *current {
            reversed.push(node.data.clone());
            current = &node.next;
        }
        reversed
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        _item
            .iter()
            .fold(SimpleLinkedList::new(), |mut list, data| {
                list.push(data.clone());
                list
            })
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut vec = vec![];
        while let Some(data) = self.pop() {
            vec.insert(0, data);
        }
        vec
    }
}
