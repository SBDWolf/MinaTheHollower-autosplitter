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
    /// Radiant Manor
    ///
    /// Split on the Radiant Manor generator
    #[default = true]
    pub game_cleared: bool,
    #[heading_level = 1]
    bosses_defeated: Title,
    #[heading_level = 2]
    required_bosses: Title,
    /// Nether Kraken
    ///
    /// Split after defeating Nether Kraken
    #[default = false]
    pub nether_kraken_defeated: bool,
    /// Hulk Trooper
    ///
    /// Split after defeating Hulk Trooper
    #[default = false]
    pub hulk_trooper_defeated: bool,
    /// Thorne 1
    ///
    /// Split after defeating Thorne 1
    #[default = false]
    pub thorne_1_defeated: bool,
    /// The Duchess
    ///
    /// Split after defeating The Duchess
    #[default = false]
    pub the_duchess_defeated: bool,
    /// Nox's Beast
    ///
    /// Split after defeating Nox's Beast
    #[default = false]
    pub noxs_beast_defeated: bool,
    /// Madd House
    ///
    /// Split after defeating Madd House
    #[default = false]
    pub madd_house_defeated: bool,
    /// The Carving Man
    ///
    /// Split after defeating The Carving Man
    #[default = false]
    pub the_carving_man_defeated: bool,
    /// Major Miner
    ///
    /// Split after defeating Major Miner
    #[default = false]
    pub major_miner_defeated: bool,
    /// Mined Mind
    ///
    /// Split after defeating Mined Mind
    #[default = false]
    pub mined_mind_defeated: bool,
    /// Thorne 2
    ///
    /// Split after defeating Thorne 2
    #[default = false]
    pub thorne_2_defeated: bool,
    /// Frozen Horror
    ///
    /// Split after defeating Frozen Horror
    #[default = false]
    pub frozen_horror_defeated: bool,
    /// Locomotress Agnes
    ///
    /// Split after defeating Locomotress Agnes
    #[default = false]
    pub locomotress_agnes_defeated: bool,
    /// Lumenarks
    ///
    /// Split after defeating Lumenarks
    #[default = false]
    pub lumenarks_defeated: bool,
    /// The Congealed
    ///
    /// Split after defeating The Congealed
    #[default = false]
    pub the_congealed_defeated: bool,
    /// Furgus The Faithful
    ///
    /// Split after defeating Furgus The Faithful
    #[default = false]
    pub furgus_the_faithful_defeated: bool,
    /// Baron Lionel
    ///
    /// Split after defeating Baron Lionel
    #[default = false]
    pub baron_lionel_defeated: bool,
    /// Radiant Lionel
    ///
    /// Split after defeating Radiant Lionel
    #[default = false]
    pub radiant_lionel_defeated: bool,
    #[heading_level = 2]
    optional_bosses: Title,
    /// Midden
    ///
    /// Split after defeating Midden
    #[default = false]
    pub midden_defeated: bool,
    /// Maxi
    ///
    /// Split after defeating Maxi
    #[default = false]
    pub maxi_defeated: bool,
    /// Mock Moon
    ///
    /// Split after defeating Mock Moon
    #[default = false]
    pub mock_moon_defeated: bool,
    /// Dugin?
    ///
    /// Split after defeating Dugin?
    #[default = false]
    pub dugin_defeated: bool,
    /// Armand
    ///
    /// Split after defeating Armand
    #[default = false]
    pub armand_defeated: bool,
    /// Dark Deluxy
    ///
    /// Split after defeating Dark Deluxy
    #[default = false]
    pub dark_deluxy_defeated: bool,
    /// Mirren
    ///
    /// Split after defeating Mirren
    #[default = false]
    pub mirren_defeated: bool,
    /// ??????
    ///
    /// Split after defeating ??????
    #[default = false]
    pub thorne_3_defeated: bool,
    /// Evra
    ///
    /// Split after defeating Evra
    #[default = false]
    pub evra_defeated: bool,
    /// Wonder Willis
    ///
    /// Split after defeating Wonder Willis
    #[default = false]
    pub wonder_willis_defeated: bool,
    /// Thalassion
    ///
    /// Split after defeating Thalassion
    #[default = false]
    pub thalassion_defeated: bool,
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
    timer: Title,
    /// Auto-Reset/Start (on Profile Select)
    ///
    /// Automatically reset and start the timer. This will happen when selecting a new profile.
    #[default = true]
    pub auto_reset_start: bool,
}
