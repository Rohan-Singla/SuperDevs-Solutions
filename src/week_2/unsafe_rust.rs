pub fn swap_values(a: &mut i32, b: &mut i32) {
    let pa: *mut i32 = a;
    let pb: *mut i32 = b;
    unsafe {
        std::ptr::swap(pa, pb);
    }
}

pub struct SafeArray(pub Vec<i32>);

impl SafeArray {
    pub fn new(data: Vec<i32>) -> Self {
        SafeArray(data)
    }

    pub fn get(&self, i: usize) -> Option<i32> {
        if i < self.0.len() { Some(self.0[i]) } else { None }
    }

    pub unsafe fn get_unchecked(&self, i: usize) -> i32 {
        *self.0.as_ptr().add(i)
    }

    pub fn sum_all(&self) -> i32 {
        (0..self.0.len()).map(|i| unsafe { self.get_unchecked(i) }).sum()
    }
}