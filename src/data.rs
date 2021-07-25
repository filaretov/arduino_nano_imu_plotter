use ringbuffer::{AllocRingBuffer, RingBufferExt, RingBufferWrite};

const N_VALUES: usize = 256;

pub struct DataQueue {
    x: AllocRingBuffer<f64>,
    y: AllocRingBuffer<f64>,
    z: AllocRingBuffer<f64>,
}

impl DataQueue {
    pub fn new() -> Self {
        let x = AllocRingBuffer::with_capacity(N_VALUES);
        let y = AllocRingBuffer::with_capacity(N_VALUES);
        let z = AllocRingBuffer::with_capacity(N_VALUES);
        DataQueue { x, y, z }
    }

    pub fn push(&mut self, x: f64, y: f64, z: f64) {
        self.x.push(x);
        self.y.push(y);
        self.z.push(z);
    }

    pub fn get(&self) -> [Vec<(f64, f64)>; 3] {
        let time: Vec<f64> = (0..N_VALUES).map(|x| x as f64).collect();
        let x: Vec<f64> = self.x.to_vec();
        let y: Vec<f64> = self.y.to_vec();
        let z: Vec<f64> = self.z.to_vec();
        let x: Vec<(f64, f64)> = time.iter().cloned().zip(x.iter().cloned()).collect();
        let y: Vec<(f64, f64)> = time.iter().cloned().zip(y.iter().cloned()).collect();
        let z: Vec<(f64, f64)> = time.iter().cloned().zip(z.iter().cloned()).collect();

        [x, y, z]
    }
}

impl std::fmt::Debug for DataQueue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self.x)
    }
}
