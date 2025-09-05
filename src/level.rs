use lazy_static::lazy_static;
use std::sync::RwLock;

use crate::config::Config;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Level {
    Foundation,      // 筑基
    GoldenCore,      // 结丹
    NascentSoul,     // 元婴
    SpiritTransform, // 化神
    VoidRefining,    // 炼虚
    BodyIntegration, // 合体
    Mahayana,        // 大乘
}

lazy_static! {
    static ref REQUIRED_CULTIVATION: RwLock<Vec<u64>> = RwLock::new(vec![10, 100, 1000, 10000, 100000, 1000000, 10000000]);
    static ref TOTAL_LIFESPAN: RwLock<Vec<u64>> = RwLock::new(vec![100, 900, 8900, 88900, 888900, 8888900, 88888900]);
}

impl Level {
    pub fn name(&self) -> &'static str {
        match self {
            Level::Foundation => "筑基",
            Level::GoldenCore => "结丹",
            Level::NascentSoul => "元婴",
            Level::SpiritTransform => "化神",
            Level::VoidRefining => "炼虚",
            Level::BodyIntegration => "合体",
            Level::Mahayana => "大乘",
        }
    }

    pub fn idx(&self) -> usize {
        match self {
            Level::Foundation => 0,
            Level::GoldenCore => 1,
            Level::NascentSoul => 2,
            Level::SpiritTransform => 3,
            Level::VoidRefining => 4,
            Level::BodyIntegration => 5,
            Level::Mahayana => 6,
        }
    }

    pub fn required_cultivation(&self) -> u64 {
        REQUIRED_CULTIVATION.read().unwrap()[self.idx()]
    }

    pub fn total_lifespan(&self) -> u64 {
        TOTAL_LIFESPAN.read().unwrap()[self.idx()]
    }

    pub fn next_level(&self) -> Option<Level> {
        match self {
            Level::Foundation => Some(Level::GoldenCore),
            Level::GoldenCore => Some(Level::NascentSoul),
            Level::NascentSoul => Some(Level::SpiritTransform),
            Level::SpiritTransform => Some(Level::VoidRefining),
            Level::VoidRefining => Some(Level::BodyIntegration),
            Level::BodyIntegration => Some(Level::Mahayana),
            Level::Mahayana => None,
        }
    }

    pub fn update(config: &Config) {
        let total_levels = 7;
        let mut required_cultivation = vec![config.lvup.start];
        let mut total_lifespan = vec![config.lifespan.start];
        for i in 1..total_levels {
            required_cultivation.push(required_cultivation.last().unwrap() + config.lvup.diff(i));
            total_lifespan.push(total_lifespan.last().unwrap() + config.lifespan.diff(i));
        }
        *REQUIRED_CULTIVATION.write().unwrap() = required_cultivation;
        *TOTAL_LIFESPAN.write().unwrap() = total_lifespan;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifespan_calc() {
        let original;
        {
            original = TOTAL_LIFESPAN.read().unwrap().clone();
        }
        Level::update(&Config::default());
        let current;
        {
            current =  TOTAL_LIFESPAN.read().unwrap().clone();
        }
        assert_eq!(original, current)
    }

    #[test]
    fn test_cult_requirements_calc() {
        let original;
        {
            original = REQUIRED_CULTIVATION.read().unwrap().clone();
        }
        Level::update(&Config::default());
        let current;
        {
            current =  REQUIRED_CULTIVATION.read().unwrap().clone();
        }
        assert_eq!(original, current)
    }

}
