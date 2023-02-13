use std::collections::VecDeque;

pub type Value = isize;

#[derive(Debug, Clone)]
pub struct Decryptor {
    values: Vec<Value>,
    indices: VecDeque<usize>,
}

impl Decryptor {
    pub fn new(values: Vec<Value>, decryption_key: Value) -> Self {
        let values: Vec<_> = values
            .into_iter()
            .map(|value| value * decryption_key)
            .collect();

        let indices = (0..values.len()).collect();

        Self { values, indices }
    }

    pub fn decrypt(&mut self, iterations: usize) -> [Value; 3] {
        self.mix(iterations);

        let zero = self
            .indices
            .iter()
            .position(|&i| self.values[i] == 0)
            .unwrap();

        [1000, 2000, 3000].map(|i| self.values[self.indices[(zero + i) % self.values.len()]])
    }

    fn mix(&mut self, iterations: usize) {
        let n = self.values.len() as isize;

        for _ in 0..iterations {
            for (key, &value) in self.values.iter().enumerate() {
                let i = self.indices.iter().position(|&i| i == key).unwrap();
                self.indices.rotate_left(i);

                let popped = self.indices.pop_front().unwrap();
                let shift = value.rem_euclid(n - 1) as usize;

                self.indices.rotate_left(shift);
                self.indices.push_front(popped);
            }
        }
    }
}
