struct LinkedList {
    /* Put some fields in here */
}

impl LinkedList {
    fn new() -> LinkedList {
        /* Implement me */
    }

    fn insert(&mut self, data: u32) {
        /* Implement me */
    }

    fn delete(&mut self, data: u32) {
        /* Implement me */
    }

    fn get(&mut self, index: u32) -> u32 {
        /* Implement me */
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn linked_list_insert_test() {
        let mut ll = LinkedList::new();

        ll.insert(10);
        assert_eq!(ll.get(0), 10);
    }

    #[test]
    fn linked_list_delete_test() {
        let mut ll = LinkedList::new();

        ll.insert(20);
        ll.insert(30);

        ll.delete(20);
        assert_eq!(ll.get(0), 30);
    }
}
