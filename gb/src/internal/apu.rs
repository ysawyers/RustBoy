// NRxy: nr0-4 IS THE REGISTER ID AND THE INDEX [X] IS THE CHANNEL
pub struct APU {
    prev_div_apu_bit: u8,
    div_apu_counter: u8,

    nr1: [u8; 5],
    nr2: [u8; 5],
    nr3: [u8; 5],
    nr4: [u8; 5],
    nr5: [u8; 5],

    ch1_dac: bool,

    /* NR11 */
    ch1_wave_duty: u8, // controls output waveform
    ch1_initial_length_timer: u8, // controls how long channel stays alive until being shut down automatically
    
    /* NR12 */
    ch1_initial_volume: u8, // bits 7-4
    ch1_envelope_direction: u8, // bit 3
    ch1_sweep_pace: u8, // bits 2 - 0

    /* NR14 */
    ch1_period_upper: u8, // upper 3 bits
    ch1_length_enable: bool, // when enabled tick the length
    ch1_length_timer_lock: bool, // obscure behavior when length timer overflows: lock incrementing after overflow and channel gets disabled until channel is retriggered or length counter is written.
}

impl APU {
    pub fn write_registers(&mut self, addr: u16, val: u8) {
        match addr {
            0xFF10 => { // NR10

            },

            0xFF11 => { // NR11
                self.ch1_initial_length_timer = val & 0x1F;
                self.ch1_wave_duty = (val & 0xC0) >> 6;
                self.ch1_length_timer_lock = false;
            },
            0xFF12 => { // NR12
                self.ch1_initial_volume = (val & 0xF0) >> 4; // how loud the channel initially is
                self.ch1_envelope_direction = (val & 0x04) >> 2; // 0 - decrease volume over time | 1 - increase volume over time
                self.ch1_sweep_pace = val & 0x03; // envelope ticks at 64 Hz and channels envelope will be increased/decreased (depending on the direction) every sweep pace ? of those ticks. A setting of 0 disables the envelope  

                self.ch1_dac = (val & 0xF8) >> 3 != 0;
                if !self.ch1_dac {
                    self.nr5[2] &= !(1 << 0); // switches channel 1 off when DAC is disabled.
                }

                self.nr1[2] = val;
            },
            0xFF14 => { // NR14
                // turn channel on ONLY when dac is set and MSB is set
                if ((val >> 7) & 0x1 == 1) && self.ch1_dac {
                    self.nr5[2] |= 1 << 0;
                    self.ch1_length_timer_lock = false;
                }
                self.ch1_length_enable = ((val >> 6) & 0x1) == 1;
                self.ch1_period_upper = val & 0x07; // 3 upper bits for the whole periods value; lower 8 stored in NR13
            },

            0xFF24 => self.nr5[0] = val,
            0xFF25 => self.nr5[1] = val,
            0xFF26 => self.nr5[2] = val & 0x80, // controls whether audio is on or off

            _ => ()
        }
    }

    pub fn read_registers(&self, addr: u16) -> u8 {
        match addr {
            /* CHANNEL 1 */
            // 0xFF10 => {


            //     return 0
            // },

            0xFF11 => self.ch1_wave_duty, // 00 - 12.5% | 01 - 25% | 10 - 50% | 11 - 75% (output waveform)
            0xFF12 => self.nr1[2],

            // 0xFF13 => {



            //     return 0
            // },

            0xFF14 => if self.ch1_length_enable { 1 } else { 0 },

            0xFF24 => self.nr5[0], // bits 6-4 left volume | bits 2-0 right volume
            0xFF25 => self.nr5[1],
            0xFF26 => self.nr5[2], // bit 3 - CH4 on? | bit 2 - CH3 on? | bit 1 - CH2 on? | bit 0 - CH1 on?

            _ => 0x00
        }
    }

    pub fn update(&mut self, current_div_apu_bit: u8) {
        if self.prev_div_apu_bit == 1 && current_div_apu_bit == 0 {
            self.div_apu_counter = self.div_apu_counter.wrapping_add(1);

            // Envelope sweep
            if self.div_apu_counter % 8 == 0 {

            }

            // Sound length
            if self.div_apu_counter % 2 == 0 {
                if self.ch1_length_enable && !self.ch1_length_timer_lock {
                    self.ch1_initial_length_timer += 1;

                    if self.ch1_initial_length_timer == 64 {
                        self.nr5[2] &= !(1 << 0); // switches channel 1 off when length timer gets overflowed.
                        self.ch1_initial_length_timer = 0;
                        self.ch1_length_timer_lock = true;
                    }
                }
            }

            // CH1 freq sweep
            if self.div_apu_counter % 4 == 0 {

            }
        }
        self.prev_div_apu_bit = current_div_apu_bit;
    }
}

impl Default for APU {
    fn default() -> Self {
        Self {
            prev_div_apu_bit: 0,
            div_apu_counter: 0,

            nr1: [0x0; 5],
            nr2: [0x0; 5],
            nr3: [0x0; 5],
            nr4: [0x0; 5],
            nr5: [0x0; 5],

            ch1_dac: false,
            ch1_wave_duty: 0x00,
            ch1_initial_length_timer: 0x00,
            ch1_initial_volume: 0x0,
            ch1_envelope_direction: 0x0,
            ch1_sweep_pace: 0x0,
            ch1_period_upper: 0x00,
            ch1_length_enable: false,
            ch1_length_timer_lock: false
        }
    }
}