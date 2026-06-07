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
    // #[heading_level = 1]
    // bosses: Title,
    // #[default = true]
    // pub lionel_two: bool,
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
    pub noxs_bayou: bool,
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
    enter_area: Title,
    /// Astral Orrery
    ///
    /// Split when entering Astral Orrery for the first time
    #[default = false]
    pub astral_orrery_enter: bool,
    /// Backwaters
    ///
    /// Split when entering Backwaters for the first time
    #[default = false]
    pub backwaters_enter: bool,
    /// Nox's Bayou
    ///
    /// Split when entering Nox's Bayou for the first time
    #[default = false]
    pub noxs_bayou_enter: bool,
    /// Sandfalls
    ///
    /// Split when entering Sandfalls for the first time
    #[default = false]
    pub sandfalls_enter: bool,
    /// Bone Beach
    ///
    /// Split when entering Bone Beach for the first time
    #[default = false]
    pub bone_beach_enter: bool,
    /// Mourner's Mile
    ///
    /// Split when entering Mourner's Mile for the first time
    #[default = false]
    pub mourners_mile_enter: bool,
    /// Queensbury Crypt
    ///
    /// Split when entering Queensbury Crypt for the first time
    #[default = false]
    pub queensbury_crypt_enter: bool,
    /// Eastern Heath: Grassland
    ///
    /// Split when entering Eastern Heath for the first time
    #[default = false]
    pub eastern_heath_enter: bool,
    /// Coltrane Peak
    ///
    /// Split when entering Coltrane Peak for the first time
    #[default = false]
    pub coltrane_peak_enter: bool,
    /// Loner's Landing
    ///
    /// Split when entering Loner's Landing for the first time
    #[default = false]
    pub loners_landing_enter: bool,
    /// Radiant Manor Foyer: Grand Hall
    ///
    /// Split when entering Radiant Manor Foyer for the first time
    #[default = false]
    pub radiant_manor_foyer_enter: bool,
    /// Radiant Manor
    ///
    /// Split when entering Radiant Manor for the first time
    #[default = false]
    pub radiant_manor_enter: bool,
    /// Ossex
    ///
    /// Split when entering Ossex for the first time
    #[default = false]
    pub ossex_enter: bool,
    /// Kindlewood
    ///
    /// Split when entering Kindlewood for the first time
    #[default = false]
    pub kindlewood_enter: bool,
    /// Septemburg
    ///
    /// Split when entering Septemburg for the first time
    #[default = false]
    pub septemburg_enter: bool,
    /// Southern Outskirts
    ///
    /// Split when entering Southern Outskirts for the first time
    #[default = false]
    pub southern_outskirts_enter: bool,
    /// Western Wilds
    ///
    /// Split when entering Western Wilds for the first time
    #[default = false]
    pub western_wilds_enter: bool,
    #[heading_level = 1]
    game_clared: Title,
    /// Game Cleared
    ///
    /// Split when the game is cleared
    #[default = true]
    pub game_cleared: bool,
}
