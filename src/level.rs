#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Level {
    QiRefining = 0,      // 炼气
    Foundation = 1,      // 筑基
    GoldenCore = 2,      // 结丹
    NascentSoul = 3,     // 元婴
    SpiritTransform = 4, // 化神
    VoidRefining = 5,    // 炼虚
    BodyIntegration = 6, // 合体
    Mahayana = 7,        // 大乘
}

impl Level {
    pub fn name(&self) -> &'static str {
        match self {
            Level::QiRefining => "炼气",
            Level::Foundation => "筑基",
            Level::GoldenCore => "结丹",
            Level::NascentSoul => "元婴",
            Level::SpiritTransform => "化神",
            Level::VoidRefining => "炼虚",
            Level::BodyIntegration => "合体",
            Level::Mahayana => "大乘",
        }
    }

    pub fn required_cultivation(&self) -> u64 {
        match self {
            Level::QiRefining => 0,
            Level::Foundation => 10,
            Level::GoldenCore => 100,
            Level::NascentSoul => 1000,
            Level::SpiritTransform => 10000,
            Level::VoidRefining => 100000,
            Level::BodyIntegration => 1000000,
            Level::Mahayana => 10000000,
        }
    }

    pub fn total_lifespan(&self) -> u64 {
        match self {
            Level::QiRefining => 100,
            Level::Foundation => 100,
            Level::GoldenCore => 900,          // 100 + 800
            Level::NascentSoul => 8900,        // 900 + 8000
            Level::SpiritTransform => 88900,   // 8900 + 80000
            Level::VoidRefining => 888900,     // 88900 + 800000
            Level::BodyIntegration => 8888900, // 888900 + 8000000
            Level::Mahayana => 88888900,       // 8888900 + 80000000
        }
    }

    pub fn next_level(&self) -> Option<Level> {
        match self {
            Level::QiRefining => Some(Level::Foundation),
            Level::Foundation => Some(Level::GoldenCore),
            Level::GoldenCore => Some(Level::NascentSoul),
            Level::NascentSoul => Some(Level::SpiritTransform),
            Level::SpiritTransform => Some(Level::VoidRefining),
            Level::VoidRefining => Some(Level::BodyIntegration),
            Level::BodyIntegration => Some(Level::Mahayana),
            Level::Mahayana => None,
        }
    }
}
