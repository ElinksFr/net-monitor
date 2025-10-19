use std::{mem::MaybeUninit, ops::Index};

#[derive(Debug)]
pub struct HistoryBuffer<const N: usize, T> {
    buffer: [MaybeUninit<T>; N],
    start_index: usize,
    size: usize,
}

/// Ring buffer with at least one element
impl<const N: usize, T> HistoryBuffer<N, T> {
    pub fn init(item: T) -> HistoryBuffer<N, T> {
        let mut buffer = [const { MaybeUninit::uninit() }; N];
        buffer[0] = MaybeUninit::new(item);
        HistoryBuffer {
            buffer,
            start_index: 0,
            size: 1,
        }
    }

    pub fn last(&self) -> &T {
        &self[self.size - 1]
    }

    pub fn push(&mut self, item: T) {
        if self.size < N {
            self.buffer[self.size] = MaybeUninit::new(item);
            self.size += 1;
        } else {
            // Buffer is full, we override old values
            let insertion_index = (self.start_index + self.size) % N;
            self.buffer[insertion_index] = MaybeUninit::new(item);
            self.start_index = insertion_index + 1;
        }
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index > self.size {
            None
        } else {
            Some(unsafe { self.buffer[(self.start_index + index) % N].assume_init_ref() })
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }
}

impl<const N: usize, T> Index<usize> for HistoryBuffer<N, T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out of band")
    }
}

pub struct HistoryBufferIterator<'a, const N: usize, T> {
    yielded_items: usize,
    buffer: &'a HistoryBuffer<N, T>,
}

impl<'a, const N: usize, T> IntoIterator for &'a HistoryBuffer<N, T> {
    type Item = &'a T;

    type IntoIter = HistoryBufferIterator<'a, N, T>;

    fn into_iter(self) -> Self::IntoIter {
        HistoryBufferIterator {
            yielded_items: 0,
            buffer: self,
        }
    }
}

impl<'a, const N: usize, T> Iterator for HistoryBufferIterator<'a, N, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.yielded_items >= self.buffer.size {
            None
        } else {
            let tmp = &self.buffer[self.yielded_items];
            Some(tmp)
        };
        self.yielded_items += 1;
        result
    }
}

impl<'a, const N: usize, T> DoubleEndedIterator for HistoryBufferIterator<'a, N, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let result = if self.yielded_items >= self.buffer.size {
            None
        } else {
            let tmp = &self.buffer[self.buffer.size - (self.yielded_items + 1)];
            Some(tmp)
        };
        self.yielded_items += 1;
        result
    }
}

#[cfg(test)]
mod tests_buffer {
    use super::HistoryBuffer;

    #[test]
    fn test_default_size_of_buffer_is_1() {
        let buffer = HistoryBuffer::<10, u8>::init(4);

        assert_eq!(buffer.size, 1);
    }

    #[test]
    fn test_pushing_element_increase_the_size() {
        let mut buffer = HistoryBuffer::<10, u8>::init(1);
        buffer.push(2);

        assert_eq!(buffer.size, 2);
    }

    #[test]
    fn test_wrapping_when_max_size_is_reached() {
        let mut buffer = HistoryBuffer::<1, u8>::init(1);
        buffer.push(2);

        assert_eq!(buffer.size, 1);
        assert_eq!(buffer[0], 2);
    }

    #[test]
    fn test_iter_buffer_not_at_max_capacity() {
        let mut buffer = HistoryBuffer::<3, u8>::init(1);
        buffer.push(2);

        let result: Vec<_> = buffer.into_iter().collect();

        assert_eq!(result.len(), 2);
        assert_eq!(*result[0], 1);
        assert_eq!(*result[1], 2);
    }

    #[test]
    fn test_iter_buffer_at_max_capacity() {
        let mut buffer = HistoryBuffer::<3, u8>::init(1);
        buffer.push(2);
        buffer.push(3);

        let result: Vec<_> = buffer.into_iter().collect();

        assert_eq!(result.len(), 3);
        assert_eq!(*result[0], 1);
        assert_eq!(*result[1], 2);
        assert_eq!(*result[2], 3);
    }

    #[test]
    fn test_iter_buffer_at_max_capacity_that_wrapped_around() {
        let mut buffer = HistoryBuffer::<3, u8>::init(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        let result: Vec<_> = buffer.into_iter().collect();

        assert_eq!(result.len(), 3);
        assert_eq!(*result[0], 2);
        assert_eq!(*result[1], 3);
        assert_eq!(*result[2], 4);
    }

    #[test]
    fn test_reverse_iter_buffer_not_at_max_capacity() {
        let mut buffer = HistoryBuffer::<3, u8>::init(1);
        buffer.push(2);

        let result: Vec<_> = buffer.into_iter().rev().collect();

        assert_eq!(result.len(), 2);
        assert_eq!(*result[0], 2);
        assert_eq!(*result[1], 1);
    }

    #[test]
    fn test_reverse_iter_buffer_at_max_capacity_that_wrapped_around() {
        let mut buffer = HistoryBuffer::<3, u8>::init(1);
        buffer.push(2);
        buffer.push(3);
        buffer.push(4);

        let result: Vec<_> = buffer.into_iter().rev().collect();

        assert_eq!(result.len(), 3);
        assert_eq!(*result[0], 4);
        assert_eq!(*result[1], 3);
        assert_eq!(*result[2], 2);
    }
}
