use fmod_studio;

pub use audio::update::AudioUpdate;

pub mod update;

pub struct System {
    pub studio: fmod_studio::system::System,
}

impl Default for System {
    fn default() -> Self {
        let mut studio = fmod_studio::system::System::new(512, true).unwrap();
        studio.load_bank_file("resources/audio/fmod/Desktop/Master Bank.bank", false).unwrap();
        studio.load_bank_file("resources/audio/fmod/Desktop/Master Bank.strings.bank", false).unwrap();
        System { studio }
    }
}
