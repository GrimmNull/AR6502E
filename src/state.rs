

pub mod state_type {
    use console::Term;

    #[derive(Clone)]
    pub struct State {
        pub memory: Vec<u8>,    // 512bit memory
        pub stack: Vec<u8>,

        // registers
        pub pc: i16,    // program counter
        pub ac: i8,     // accumulator
        pub x: i8,      // X register
        pub y: i8,      // Y register
        pub sp: i8,     // stack pointer
        pub sr: i8,     // status register

        // flags
        pub n: bool,    // negative
        pub v: bool,    // overflow
        pub ig: bool,   // ignore
        pub b: bool,    // break
        pub d: bool,    // decimal
        pub i: bool,    // interrupt
        pub z: bool,    // zero
        pub c: bool     // carry
    }

    impl State {
        pub fn print_memory(&self, page: u8) {

            let memory_page = if page == 0 {(0..self.memory.len() as u8 -1)} else {((page-1)*64..page*64-1)};
            for line_index in memory_page.step_by(8) {
                for column_index in line_index..line_index+8 {
                    print!("{memory_cell} ", memory_cell = self.memory[column_index as usize].clone());
                }
                println!();
            }
        }

        pub fn print_registers(&self) {
            let term = Term::stdout();

                println!("Program counter: {pc}", pc = self.pc);
                println!("Accumulator: {ac}", ac = self.ac);
                println!("X registry: {x}", x = self.x);
                println!("Y registry: {y}", y = self.y);
                println!("Stack pointer: {sp}", sp = self.sp);
                println!("Status register: {sr}", sr = self.sr);
        }

        pub fn print_flags(&self) {
                println!("Negative: {n}", n = self.n);
                println!("Overflow: {v}", v = self.v);
                println!("Ignore: {ig}", ig = self.ig);
                println!("Break: {b}", b = self.b);
                println!("Decimal: {d}", d = self.d);
                println!("Interrupt: {i}", i = self.i);
                println!("Zero: {z}", z = self.z);
                println!("Carry: {c}", c = self.c);
        }

        pub fn print_state(&self) {
            println!("Registers:");
            self.print_registers();
            println!();

            println!("Flags:");
            self.print_flags();
            println!();

            println!("Memory: ");
            self.print_memory(0);
        }
    }

    pub fn get_initial_state() -> State {
        return State{
            memory: vec![0; 512],
            stack: Vec::new(),
            pc: 0,
            ac: 0,
            x: 0,
            y: 0,
            sp: 0,
            sr: 0,
            n: false,
            v: false,
            ig: false,
            b: false,
            d: false,
            i: false,
            z: false,
            c: false
        }
    }
}