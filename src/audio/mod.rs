use fmod_studio;
use fmod_studio::guid::Guid;
use fmod_studio::error::FmodError;

pub use audio::update::AudioUpdate;

pub mod update;

pub struct System {
    pub studio: fmod_studio::system::System,
}

impl System {
    pub fn get_id(&self, object: &str) -> Option<Guid> {
        match self.studio.lookup_id(object) {
            Ok(guid) => Some(guid),
            Err(FmodError::EventNotFound) => None,
            Err(err) => {
                eprintln!(
                    "[WARNING] event lookup of {:?}: {:?}({})",
                    object,
                    err,
                    err.description()
                );
                None
            }
        }
    }

    pub fn play_oneoff(&self, id: &Guid) {
        self.studio
            .get_event_by_id(id)
            .and_then(|desc| desc.create_instance())
            .and_then(|inst| inst.start())
            .map_err(|err| {
                eprintln!("[WARNING] play_oneoff: {}", err.description())
            })
            .ok();
    }
}

impl Default for System {
    fn default() -> Self {
        let mut studio = fmod_studio::system::System::new(512, true).unwrap();
        studio
            .load_bank_file("resources/audio/fmod/Desktop/Master Bank.bank", false)
            .unwrap();
        studio
            .load_bank_file(
                "resources/audio/fmod/Desktop/Master Bank.strings.bank",
                false,
            )
            .unwrap();
        System { studio }
    }
}
