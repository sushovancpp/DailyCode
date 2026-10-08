use std::collections::HashMap;
use std::ptr;

struct Node {
    count: i32,
    keys: HashMap<String, ()>,
    prev: *mut Node,
    next: *mut Node,
}

impl Node {
    fn new(count: i32) -> *mut Node {
        Box::into_raw(Box::new(Node {
            count,
            keys: HashMap::new(),
            prev: ptr::null_mut(),
            next: ptr::null_mut(),
        }))
    }
}

struct AllOne {
    head: *mut Node,
    tail: *mut Node,
    map: HashMap<String, *mut Node>,
}

impl AllOne {
    fn new() -> Self {
        unsafe {
            let head = Node::new(0);
            let tail = Node::new(0);

            (*head).next = tail;
            (*tail).prev = head;

            Self {
                head,
                tail,
                map: HashMap::new(),
            }
        }
    }

    fn insert_after(&mut self, node: *mut Node, new_node: *mut Node) {
        unsafe {
            let next = (*node).next;

            (*new_node).prev = node;
            (*new_node).next = next;

            (*node).next = new_node;
            (*next).prev = new_node;
        }
    }

    fn remove_node(&mut self, node: *mut Node) {
        unsafe {
            let prev = (*node).prev;
            let next = (*node).next;

            (*prev).next = next;
            (*next).prev = prev;

            drop(Box::from_raw(node));
        }
    }

    fn inc(&mut self, key: String) {
        unsafe {
            if !self.map.contains_key(&key) {
                let first = (*self.head).next;

                let node;

                if first != self.tail && (*first).count == 1 {
                    node = first;
                } else {
                    node = Node::new(1);
                    self.insert_after(self.head, node);
                }

                (*node).keys.insert(key.clone(), ());
                self.map.insert(key, node);
                return;
            }

            let current = *self.map.get(&key).unwrap();
            let next = (*current).next;
            let new_count = (*current).count + 1;

            let target;

            if next != self.tail && (*next).count == new_count {
                target = next;
            } else {
                target = Node::new(new_count);
                self.insert_after(current, target);
            }

            (*current).keys.remove(&key);
            (*target).keys.insert(key.clone(), ());
            self.map.insert(key, target);

            if (*current).keys.is_empty() {
                self.remove_node(current);
            }
        }
    }

    fn dec(&mut self, key: String) {
        unsafe {
            let current = match self.map.get(&key) {
                Some(&node) => node,
                None => return,
            };

            let count = (*current).count;

            if count == 1 {
                (*current).keys.remove(&key);
                self.map.remove(&key);

                if (*current).keys.is_empty() {
                    self.remove_node(current);
                }

                return;
            }

            let prev = (*current).prev;
            let new_count = count - 1;

            let target;

            if prev != self.head && (*prev).count == new_count {
                target = prev;
            } else {
                target = Node::new(new_count);
                self.insert_after(prev, target);
            }

            (*current).keys.remove(&key);
            (*target).keys.insert(key.clone(), ());
            self.map.insert(key, target);

            if (*current).keys.is_empty() {
                self.remove_node(current);
            }
        }
    }

    fn get_max_key(&self) -> String {
        unsafe {
            if (*self.tail).prev == self.head {
                return String::new();
            }

            let node = (*self.tail).prev;

            if let Some(key) = (*node).keys.keys().next() {
                return key.clone();
            }

            String::new()
        }
    }

    fn get_min_key(&self) -> String {
        unsafe {
            if (*self.head).next == self.tail {
                return String::new();
            }

            let node = (*self.head).next;

            if let Some(key) = (*node).keys.keys().next() {
                return key.clone();
            }

            String::new()
        }
    }
}
