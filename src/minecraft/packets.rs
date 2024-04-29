use std::io::Read;

use bstream::{EnumBinaryStream, Vu32LenByteSlice};
use bstream_macro::{b_enum, BStream};

use crate::minecraft::*;
use crate::minecraft::packets::PacketKind::*;

macro_rules! register_pk {
    ($name:tt,$id:tt,$c:tt,$kind:tt) => {
        impl Packet for $name {
            #[inline]
            fn compressible() -> bool {
                return $c;
            }
            #[inline]
            fn id() -> i32 {
                return $id;
            }
            #[inline]
            fn kind() -> PacketKind {
                return $kind(Default::default());
            }
        }
    };
}

pub enum PacketKind {
    RequestNetworkSettings(RequestNetworkSettingsPacket),
    NetworkSettings(NetworkSettingsPacket),
    Login(LoginPacket),
    PlayStatus(PlayStatusPacket),
    ActorEvent(ActorEventPacket),
    ActorPickRequest(ActorPickRequestPacket),
}

pub fn decode_kind(r: &mut impl Read, kind: &PacketKind) -> Result<PacketKind> {
    Ok(match kind {
        RequestNetworkSettings(pk) => RequestNetworkSettings(decode(pk, r)?),
        NetworkSettings(pk) => NetworkSettings(decode(pk, r)?),
        Login(pk) => Login(decode(pk, r)?),
        PlayStatus(pk) => { PlayStatus(decode(pk, r)?) }
        ActorEvent(pk) => { ActorEvent(decode(pk, r)?) }
        ActorPickRequest(pk) => { ActorPickRequest(decode(pk, r)?) }
    })
}

/// RequestNetworkSettingsPacket is sent by the client to request network settings, such as compression, from the server.
#[derive(Debug, Clone, Default, BStream)]
pub struct RequestNetworkSettingsPacket {
    /// client_protocol is the protocol version of the player. The player is disconnected if the protocol is
    /// incompatible with the protocol of the server.
    #[BigEndian]
    pub client_protocol: i32,
}

register_pk!(
    RequestNetworkSettingsPacket,
    0xc1,
    false,
    RequestNetworkSettings
);

#[derive(Clone, Debug, Default)]
#[b_enum(u16)]
pub enum CompressionAlgorithm {
    Zlib = 0,
    Snappy = 1,

    #[default]
    None = 255,
}

/// NetworkSettingsPacket is sent by the server to update a variety of network settings. These settings modify the
/// way packets are sent over the network stack.
#[derive(Debug, Clone, Default, BStream)]
pub struct NetworkSettingsPacket {
    /// compression_threshold is the minimum size of a packet that is compressed when sent. If the size of a
    /// packet is under this value, it is not compressed.
    /// When set to 0, all packets will be left uncompressed.
    pub compression_threshold: u16,
    /// compression_algorithm is the algorithm that is used to compress packets.
    pub compression_algorithm: CompressionAlgorithm,

    /// client_throttle regulates whether the client should throttle players when exceeding of the threshold. Players
    /// outside threshold will not be ticked, improving performance on low-end devices.
    pub client_throttle: bool,
    /// client_throttle_threshold is the threshold for client throttling. If the number of players exceeds this value, the
    /// client will throttle players.
    pub client_throttle_threshold: u8,
    /// client_throttle_scalar is the scalar for client throttling. The scalar is the amount of players that are ticked
    /// when throttling is enabled.
    pub client_throttle_scalar: f32,
}

register_pk!(NetworkSettingsPacket, 0x8f, false, NetworkSettings);

/// LoginPacket is sent when the client initially tries to join the server. It is the first packet sent and contains
/// information specific to the player.
#[derive(Debug, Clone, Default, BStream)]
pub struct LoginPacket {
    /// client_protocol is the protocol version of the player. The player is disconnected if the protocol is incompatible
    /// with the protocol of the server. It has been superseded by the protocol version sent in the
    /// RequestNetworkSettings packet, so this should no longer be used by the server.
    pub client_protocol: i32,
    /// connection_request is a string containing information about the player and JWTs that may be used to
    /// verify if the player is connected to XBOX Live. The connection request also contains the necessary
    /// client public key to initiate encryption.
    pub connection_request: Vu32LenByteSlice,
}

register_pk!(LoginPacket, 0x01, true, Login);

#[derive(Debug, Clone, Default)]
#[b_enum(i32, BigEndian)]
pub enum PlayStatus {
    #[default]
    LoginSuccess = 0,
    LoginFailedClient = 1,
    LoginFailedServer = 2,
    PlayerSpawn = 3,
    LoginFailedInvalidTenant = 4,
    LoginFailedVanillaEdu = 5,
    LoginFailedEduVanilla = 6,
    LoginFailedServerFull = 7,
    LoginFailedEditorVanilla = 8,
    LoginFailedVanillaEditor = 9,
}

/// PlayStatus is sent by the server to update a player on the play status. This includes failed statuses due
/// to a mismatched version, but also success statuses.
#[derive(Debug, Clone, Default, BStream)]
pub struct PlayStatusPacket {
    /// status is the status of the packet. It is one of the constants found above.
    status: PlayStatus,
}

register_pk!(PlayStatusPacket, 0x02, true, PlayStatus);

#[derive(Debug, Clone, Default)]
#[b_enum(u64, Varint)]
pub enum ActorEvent {
    #[default]
    JUMP = 1,
    HurtAnimation = 2,
    DeathAnimation = 3,
    ArmSwing = 4,
    StopAttack = 5,
    TameFail = 6,
    TameSuccess = 7,
    ShakeWet = 8,
    UseItem = 9,
    EatGrassAnimation = 10,
    FishHookBubble = 11,
    FishHookPosition = 12,
    FishHookHook = 13,
    FishHookTease = 14,
    SquidInkCloud = 15,
    ZombieVillagerCure = 16,
    PlayAmbientSound = 17,
    RESPAWN = 18,
    IronGolemOfferFlower = 19,
    IronGolemWithdrawFlower = 20,
    LoveParticles = 21, //breeding
    VillagerAngry = 22,
    VillagerHappy = 23,
    WitchSpellParticles = 24,
    FireworkParticles = 25,
    InLoveParticles = 26,
    SilverfishSpawnAnimation = 27,
    GuardianAttack = 28,
    WitchDrinkPotion = 29,
    WitchThrowPotion = 30,
    MinecartTntPrimeFuse = 31,
    CreeperPrimeFuse = 32,
    AirSupplyExpired = 33,
    PlayerAddXpLevels = 34,
    ElderGuardianCurse = 35,
    AgentArmSwing = 36,
    EnderDragonDeath = 37,
    DustParticles = 38, //not sure what this is
    ArrowShake = 39,

    EatingItem = 57,

    BabyAnimalFeed = 60, //green particles, like bonemeal on crops
    DeathSmokeCloud = 61,
    CompleteTrade = 62,
    RemoveLeash = 63, //data 1 = cut leash
    CaravanUpdated = 64,
    ConsumeTotem = 65,
    PlayerCheckTreasureHunterAchievement = 66, //mojang...
    EntitySpawn = 67, //used for MinecraftEventing stuff, not needed
    DragonPuke = 68, //they call this puke particles
    ItemEntityMerge = 69,
    StartSwim = 70,
    BalloonPop = 71,
    TreasureHunt = 72,
    AgentSummon = 73,
    ChargedItem = 74,
    FALL = 75,
    GrowUp = 76,
    VibrationDetected = 77,
    DrinkMilk = 78,
}

/// ActorEventPacket is sent by the server when a particular event happens that has to do with an entity. Some of
/// these events are entity-specific, for example a wolf shaking itself dry, but others are used for each
/// entity, such as dying.
#[derive(Debug, Clone, Default, BStream)]
pub struct ActorEventPacket {
    /// entity_runtime_id is the runtime ID of the entity. The runtime ID is unique for each world session, and
    /// entities are generally identified in packets using this runtime ID.
    pub entity_runtime_id: ActorEvent,
    /// event_type is the ID of the event to be called. It is one of the constants that can be found above.
    pub event_type: u8,
    /// event_data is optional data associated with a particular event. The data has a different function for
    /// different event,s, however most events don't use this field at all.
    #[Varint]
    pub event_data: u32,
}

register_pk!(ActorEventPacket, 0x1b, true, ActorEvent);

/// ActorPickRequest is sent by the client when it tries to pick an entity, so that it gets a spawn egg which
/// can spawn that entity.
#[derive(Debug, Clone, Default, BStream)]
pub struct ActorPickRequestPacket {
    /// entity_unique_id is the unique ID of the entity that was attempted to be picked. The server must find the
    /// type of that entity and provide the correct spawn egg to the player.
    pub entity_unique_id: i64,
    /// hot_bar_slot is the held hot bar slot of the player at the time of trying to pick the entity. If empty,
    /// the resulting spawn egg should be put into this slot.
    pub hot_bar_slot: i8,
    /// with_data is true if the pick request requests the entity metadata.
    pub with_data: bool,
}

register_pk!(ActorPickRequestPacket, 0x23, true, ActorPickRequest);