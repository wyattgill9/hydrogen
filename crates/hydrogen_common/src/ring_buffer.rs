use std::sync::atomic::{AtomicUsize, Ordering};
use std::cell::UnsafeCell;

pub struct LockFreeRingBuffer<T> {
    buffer: Vec<UnsafeCell<Option<T>>>,
    capacity: usize,
    head: AtomicUsize,
    tail: AtomicUsize,
}

impl<T> LockFreeRingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        // Ensure capacity is a power of 2 for efficient modulo operations
        let actual_capacity = capacity.next_power_of_two();
        
        LockFreeRingBuffer {
            buffer: (0..actual_capacity)
                .map(|_| UnsafeCell::new(None))
                .collect(),
            capacity: actual_capacity,
            head: AtomicUsize::new(0),
            tail: AtomicUsize::new(0),
        }
    }

    pub fn push(&self, item: T) -> Result<(), T> {
        let mut tail = self.tail.load(Ordering::Acquire);
        
        loop {
            let head = self.head.load(Ordering::Acquire);
            
            // Check if buffer is full
            if tail.wrapping_sub(head) >= self.capacity {
                return Err(item);
            }
            
            // Try to update tail
            match self.tail.compare_exchange_weak(
                tail, 
                tail.wrapping_add(1), 
                Ordering::Release, 
                Ordering::Relaxed
            ) {
                Ok(_) => {
                    // Safe because we've claimed this slot
                    unsafe {
                        let slot = self.buffer[tail & (self.capacity - 1)].get();
                        *slot = Some(item);
                    }
                    return Ok(());
                }
                Err(current) => tail = current,
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        let mut head = self.head.load(Ordering::Acquire);
        
        loop {
            let tail = self.tail.load(Ordering::Acquire);
            
            // Check if buffer is empty
            if head == tail {
                return None;
            }
            
            // Try to update head
            match self.head.compare_exchange_weak(
                head, 
                head.wrapping_add(1), 
                Ordering::Release, 
                Ordering::Relaxed
            ) {
                Ok(_) => {
                    // Safe because we've claimed this slot
                    unsafe {
                        let slot = self.buffer[head & (self.capacity - 1)].get();
                        return slot.take();
                    }
                }
                Err(current) => head = current,
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        head == tail
    }

    pub fn len(&self) -> usize {
        let head = self.head.load(Ordering::Relaxed);
        let tail = self.tail.load(Ordering::Relaxed);
        tail.wrapping_sub(head)
    }
}

// Ensure thread-safe Send + Sync
unsafe impl<T: Send> Send for LockFreeRingBuffer<T> {}
unsafe impl<T: Send> Sync for LockFreeRingBuffer<T> {}