
// From: https://www.pcg-random.org/download.html
struct Pcg32_random_t {
    state: u64,
    inc: u64,
}

impl Pcg32_random_t {

    const fn pcg32_random_r( &mut self ) -> i32 {
        
        let old_state = self.state;
        
        self.state = ( old_state as u128 * 6364136223846793005_u128 ) as u64
                   + ( self.inc | 1 );

        let xor_shifted = ((( old_state >> 18 ) ^ old_state ) >> 27 ) as i32;
        let rot         = ( old_state >> 59 ) as i32;

        ( xor_shifted >> rot ) | ( xor_shifted << (( -rot) & 31 ))
    }
}

pub struct Rand_generator {
    pcg32_struct: Pcg32_random_t,
}

impl Rand_generator {

    pub const fn new( state: u64 ) -> Self {

        let inc          = state & 65535;
        let pcg32_struct = Pcg32_random_t { state, inc };

        Self { pcg32_struct }
    }
}

pub const fn random_i32( rand_generator: &mut Rand_generator ) -> i32 {
    rand_generator.pcg32_struct.pcg32_random_r()
}

pub const fn random_f32( rand_generator: &mut Rand_generator ) -> f32 {

    // From: https://stackoverflow.com/a/38425898

    let rand_u32 = u32::from_le_bytes(
        rand_generator.pcg32_struct.pcg32_random_r().to_be_bytes()
    );

    let float_num = f32::from_bits(( 127 << 23 ) | ( rand_u32 >> 9 ));

    ( float_num - 1.0 )
}

pub const fn random_f64( rand_generator: &mut Rand_generator ) -> f64 {

    // random f64 for poor folks

    let float_num = random_f32( rand_generator );
    float_num as f64
}
