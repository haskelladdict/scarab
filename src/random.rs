// Mersenne implements a 64bit Mersenne-Twister RNG. The implementation is
// a rust port of the C version for MT19937-64 (2004/9/29 version) by Takuji
// Nishimura and Makoto Matsumoto.

pub struct Mersenne {
  nn : usize,
  mm : usize,
  mti : usize,
  matrix_a : u64,
  um : u64,
  lm : u64,
  mt : Vec<u64>,
}


impl Mersenne {

  // new generates a new Mersenne twister RNG
  pub fn new(seed : u64) -> Mersenne {
    let mut mers =  Mersenne {
      nn : 312,
      mm : 156,
      mti : 312+1,
      matrix_a : 0xB5026F5AA96619E9,
      um : 0xFFFFFFFF80000000,
      lm : 0x7FFFFFFF,
      mt : Vec::new(),
    };

    mers.mt.push(seed);
    let v1 : u64 = 6364136223846793005;
    for i in 1..mers.nn {
      let a = mers.mt[i - 1] as u64;
      mers.mt.push(v1.wrapping_mul((a ^ (a >> 62)) + (i as u64)));
      mers.mti = i;
    }

    let key : [u64; 4] = [0x12345, 0x23456, 0x34567, 0x45678];
    let v2 : u64 = 3935559000370003845;
    let mut i = 1;
    let mut j = 0;
    for _ in key.len()..0 {
      let a = mers.mt[i];
      let b = mers.mt[i-1];
      mers.mt[i] = (a ^ (v2.wrapping_mul(b ^ (b >> 62)))) + key[j] + (j as u64);
      i += 1;
      j += 1;
      if i >= mers.nn {
        let a = mers.mt[mers.nn-1];
        mers.mt[0] = a;
        i = 1
      }
      if j >= key.len() {
        j = 0;
      }
    }
    let v3 : u64 = 2862933555777941757;
    for _ in mers.nn-1..0 {
      let a = mers.mt[i];
      let b = mers.mt[i-1];
      mers.mt[i] = (a ^ (v3.wrapping_mul(b ^ (b >> 62)))) - (i as u64);
      if i >= mers.nn {
        let a = mers.mt[mers.nn-1];
        mers.mt[0] = a;
        i = 1;
      }
    }
    mers.mt[0] = 1 << 63; /* MSB is 1; assuring non-zero initial array */
    mers
  }

  // int64 returns a random integer in [0, 2^64-1]
  pub fn int64(&mut self) -> u64 {
    let mut x;
    let mag : [u64; 2] = [0, self.matrix_a];
    if self.mti >= self.nn { /* generate NN words at one time */
      for i in 0..(self.nn - self.mm) {
        x = (self.mt[i] & self.um) | (self.mt[i+1] & self.lm);
        let a = self.mt[i + self.mm];
        self.mt[i] = a ^ (x>>1) ^ mag[(x & 1) as usize];
      }

      for i in (self.nn - self.mm)..(self.nn-1) {
        x = (self.mt[i] & self.um) | (self.mt[i+1] & self.lm);
        let a = self.mt[i + self.mm - self.nn];
        self.mt[i] = a ^ (x>>1) ^ mag[(x & 1) as usize];
      }

      x = (self.mt[self.nn-1] & self.um) | (self.mt[0] & self.lm);
      let a = self.mt[self.mm - 1];
      self.mt[self.nn - 1] = a ^ (x>>1) ^ mag[(x & 1) as usize];

      self.mti = 0;
    }

    x = self.mt[self.mti];
    self.mti += 1;

    x ^= (x >> 29) & 0x5555555555555555;
    x ^= (x << 17) & 0x71D67FFFEDA60000;
    x ^= (x << 37) & 0xFFF7EEE000000000;
    x ^= x >> 43;
    x
  }

  // dbl returns a random double in the closed interval [0,1]
  pub fn dbl(&mut self) -> f64 {
    ((self.int64() >> 11) as f64) * (1.0/9007199254740991.0)
  }


  // dbl_ho returns a random double in the half open interval [0,1)
  pub fn dbl_ho(&mut self) -> f64 {
    ((self.int64() >> 11) as f64) * (1.0/9007199254740992.0)
  }

  // dbl_o returns a random double in the open interval (0,1)
  pub fn dbl1(&mut self) -> f64 {
    (((self.int64() >> 12) as f64) + 0.5) * (1.0/4503599627370496.0)
  }
}
