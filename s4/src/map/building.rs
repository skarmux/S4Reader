use num_enum::TryFromPrimitive;

#[derive(Debug)]
#[repr(C, packed)]
pub struct Building {
    pos: (u16, u16),
    building: BuildingType,
    team: u8,
    occupied: bool,
    n_l1_swords: u8,
    n_l2_swords: u8,
    n_l3_swords: u8,
    n_l1_bows: u8,
    n_l2_bows: u8,
    n_l3_bows: u8,
    unknown0: u8,
    unknown1: u8,
    unknown2: u8,
    unknown3: u8,
    unknown4: u8,
    unknown5: u8,
    unknown6: u8,
}

#[derive(Debug, Copy, Clone, TryFromPrimitive)]
#[repr(u8)]
pub enum BuildingType {
    WoodcutterHut = 1,
    ForesterHut,
    Sawmill,
    StonecutterHut,
    WaterworkHut,
    FisherHut,
    HunterHut,
    Slaughterhouse,
    Mill,
    Bakery,
    GrainFarm,
    AnimalRanch,
    DonkeyRanch,
    Stonemine,
    Ironmine,
    Goldmine,
    Coalmine,
    Sulfurmine,
    Goldsmelt,
    Ironsmelt,
    Toolsmith,
    Weaponsmith,
    VehicleHall,
    Barracks,
    CharcoalMaker,
    TrainingCenter,
    HealerHut,
    AmmoMakerHut,
    GunpowderMakerHut,
    LandscapeMakerHut,
    Shipyard,
    Port,
    Marketplace,
    StorageArea,
    Vinyard,
    AgaveFarmerHut,
    TequilaMakerHut,
    BeekeeperHut,
    MeadmakerHut,
    ResidenceS,
    ResidenceM,
    ResidenceL,
    TempleSmall,
    TempleBig,
    Lookout,
    Tower,
    TowerBig,
    Castle,
    DarkMusroomFarm,
    DarkTemple,
    DarkFortress,
    PortA,
    PortB,
    PortC,
    PortD,
    PortE,
    PortF,
    ShipyardA,
    ShipyardB,
    ShipyardC,
    ShipyardD,
    ShipyardE,
    ShipyardF,
    Eyecatcher01,
    Eyecatcher02,
    Eyecatcher03,
    Eyecatcher04,
    Eyecatcher05,
    Eyecatcher06,
    Eyecatcher07,
    Eyecatcher08,
    Eyecatcher09,
    Eyecatcher10,
    Eyecatcher11,
    Eyecatcher12,
    ShipyardG,
    ShipyardH,
    PortG,
    PortH,
    ManacopterHall,
    SunflowerOilMakerHut,
    SunflowerFarm,
}
