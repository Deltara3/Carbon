pub struct Quirk {
    pub vf_reset: bool,
    pub memory: bool,
    pub display: bool,
    pub clipping: bool,
    pub shifting: bool,
    pub jumping: bool
}

impl Quirk {
    pub fn default() -> Quirk {
        return Quirk {
            vf_reset: false,
            memory: false,
            display: false,
            clipping: false,
            shifting: false,
            jumping: false
        }
    }

    pub fn c8() -> Quirk {
        return Quirk {
            vf_reset: true,
            memory: true,
            display: true,
            clipping: true,
            shifting: false,
            jumping: false
        }
    }

    pub fn sc() -> Quirk {
        return Quirk {
            vf_reset: false,
            memory: false,
            display: false,
            clipping: true,
            shifting: true,
            jumping: true
        }
    }

    pub fn xo() -> Quirk {
        return Quirk {
            vf_reset: false,
            memory: true,
            display: false,
            clipping: false,
            shifting: false,
            jumping: false
        }
    }

    pub fn load(&mut self) {
        // implement
    }

    pub fn save(&mut self) {
        // implement
    }
}