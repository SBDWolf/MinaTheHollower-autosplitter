use asr::settings::gui::{set_tooltip, Title};
use asr::settings::Gui;

/*
pub fn set_tooltips() {
    {
        /*
        set_tooltip(
            "show_completion",
            r##"Sets the variable "Completion". Can be displayed by LiveSplit as text."##,
        );
        */
    }
}
*/

#[derive(Gui)]
pub struct Settings {
    #[heading_level = 0]
    mina_the_hollower: Title,
    #[heading_level = 1]
    bosses: Title,
    #[default = true]
    pub lionel_two: bool,
    #[heading_level = 1]
    generators: Title,
    /// Queensbury Crypt
    ///
    /// Split on the Queensbury Crypt generator
    #[default = true]
    pub queensbury_crypt: bool,
    /// Nox's Bayou
    ///
    /// Split on the Nox's Bayou generator
    #[default = true]
    pub nox_bayou: bool,
    /// Septemburg
    ///
    /// Split on the Septemburg generator
    #[default = true]
    pub septemburg: bool,
    /// Bone Beach
    ///
    /// Split on the Bone Beach generator
    #[default = true]
    pub bone_beach: bool,
    /// Coltrane Peak
    ///
    /// Split on the Coltrane Peak generator
    #[default = true]
    pub coltrane_peak: bool,
    /// Astral Orrery
    ///
    /// Split on the Astral Orrery generator
    #[default = true]
    pub astral_orrery: bool,
    #[heading_level = 1]
    splits: Title,
    /// Game Cleared
    ///
    /// Split when the game is cleared
    #[default = true]
    pub game_cleared: bool,
}
