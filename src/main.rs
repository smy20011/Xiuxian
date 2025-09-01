use rand::{prelude::*, rng, thread_rng};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Level {
    QiRefining = 0,    // 炼气
    Foundation = 1,    // 筑基
    GoldenCore = 2,    // 结丹
    NascentSoul = 3,   // 元婴
    SpiritTransform = 4, // 化神
    VoidRefining = 5,  // 炼虚
    BodyIntegration = 6, // 合体
    Mahayana = 7,      // 大乘
}

impl Level {
    fn name(&self) -> &'static str {
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

    fn required_cultivation(&self) -> u64 {
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

    fn base_lifespan(&self) -> u64 {
        match self {
            Level::QiRefining => 100,
            Level::Foundation => 100,
            Level::GoldenCore => 900,      // 100 + 800
            Level::NascentSoul => 8900,    // 900 + 8000
            Level::SpiritTransform => 88900, // 8900 + 80000
            Level::VoidRefining => 888900,   // 88900 + 800000
            Level::BodyIntegration => 8888900, // 888900 + 8000000
            Level::Mahayana => 88888900,    // 8888900 + 80000000
        }
    }

    fn next_level(&self) -> Option<Level> {
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

#[derive(Debug, Clone)]
struct Cultivator {
    id: u64,
    level: Level,
    cultivation: u64,
    age: u64,
    remaining_lifespan: u64,
    courage: f64, // 勇气值 0-1
    alive: bool,
}

impl Cultivator {
    fn new(id: u64, rng: &mut ThreadRng) -> Self {
        Cultivator {
            id,
            level: Level::Foundation, // 新筑基修士
            cultivation: 10, // 刚筑基
            age: 20,
            remaining_lifespan: 80, // 100年寿命-20岁
            courage: rng.random::<f64>(), // 随机勇气值
            alive: true,
        }
    }

    fn can_advance(&self) -> bool {
        if let Some(next_level) = self.level.next_level() {
            self.cultivation >= next_level.required_cultivation()
        } else {
            false
        }
    }

    fn advance_level(&mut self) {
        if let Some(next_level) = self.level.next_level() {
            if self.cultivation >= next_level.required_cultivation() {
                let old_lifespan = self.level.base_lifespan();
                self.level = next_level;
                let new_lifespan = self.level.base_lifespan();
                self.remaining_lifespan += new_lifespan - old_lifespan;
            }
        }
    }

    fn yearly_cultivation(&mut self) {
        if !self.alive {
            return;
        }
        
        self.cultivation += 1;
        self.age += 1;
        self.remaining_lifespan = self.remaining_lifespan.saturating_sub(1);

        // 检查是否可以突破
        if self.can_advance() {
            self.advance_level();
        }

        // 检查是否寿元耗尽
        if self.remaining_lifespan == 0 {
            self.alive = false;
        }
    }

    fn calculate_win_rate(&self, opponent: &Cultivator) -> f64 {
        self.cultivation as f64 / (self.cultivation + opponent.cultivation) as f64
    }

    fn wants_to_fight(&self, opponent: &Cultivator) -> bool {
        let defeat_rate = 1.0 - self.calculate_win_rate(opponent);
        self.courage > defeat_rate
    }

    fn absorb_cultivation(&mut self, opponent_cultivation: u64) {
        let absorbed = opponent_cultivation / 10; // 吸收10%
        self.cultivation += absorbed;
    }
}

struct World {
    cultivators: Vec<Cultivator>,
    next_id: u64,
    year: u64,
    rng: ThreadRng,
    statistics: HashMap<Level, u64>,
}

impl World {
    fn new() -> Self {
        World {
            cultivators: Vec::new(),
            next_id: 1,
            year: 0,
            rng: rng(),
            statistics: HashMap::new(),
        }
    }

    fn add_new_cultivators(&mut self) {
        // 每年1000人筑基成功
        for _ in 0..1000 {
            let cultivator = Cultivator::new(self.next_id, &mut self.rng);
            self.cultivators.push(cultivator);
            self.next_id += 1;
        }
    }

    fn encounter_and_battle(&mut self) {
        let mut battles = Vec::new();
        
        // 统计各境界修士数量
        let mut level_counts = HashMap::new();
        let total_cultivators = self.cultivators.iter().filter(|c| c.alive).count() as u64;
        
        for cultivator in &self.cultivators {
            if cultivator.alive {
                *level_counts.entry(cultivator.level).or_insert(0) += 1;
            }
        }

        // 计算遭遇概率并安排战斗
        for i in 0..self.cultivators.len() {
            if !self.cultivators[i].alive {
                continue;
            }
            
            let level = self.cultivators[i].level;
            let nk = level_counts.get(&level).unwrap_or(&0);
            let encounter_probability = (*nk as f64) / (total_cultivators as f64);
            
            if self.rng.random::<f64>() < encounter_probability {
                // 寻找同级对手
                let mut possible_opponents = Vec::new();
                for j in 0..self.cultivators.len() {
                    if i != j && self.cultivators[j].alive && self.cultivators[j].level == level {
                        possible_opponents.push(j);
                    }
                }
                
                if !possible_opponents.is_empty() {
                    let opponent_idx = possible_opponents[self.rng.gen_range(0..possible_opponents.len())];
                    battles.push((i, opponent_idx));
                }
            }
        }

        // 执行战斗
        let mut defeated = Vec::new();
        for (idx1, idx2) in battles {
            if defeated.contains(&idx1) || defeated.contains(&idx2) {
                continue; // 已经死亡的修士不能再战斗
            }
            
            let wants_fight_1 = self.cultivators[idx1].wants_to_fight(&self.cultivators[idx2]);
            let wants_fight_2 = self.cultivators[idx2].wants_to_fight(&self.cultivators[idx1]);
            
            if wants_fight_1 || wants_fight_2 {
                let win_rate_1 = self.cultivators[idx1].calculate_win_rate(&self.cultivators[idx2]);
                let battle_result = self.rng.random::<f64>();
                
                if battle_result < win_rate_1 {
                    // idx1 胜利
                    let opponent_cultivation = self.cultivators[idx2].cultivation;
                    self.cultivators[idx1].absorb_cultivation(opponent_cultivation);
                    self.cultivators[idx2].alive = false;
                    defeated.push(idx2);
                    // println!("战斗！修士{}({})击败修士{}({})", 
                    //     self.cultivators[idx1].id, self.cultivators[idx1].level.name(),
                    //     self.cultivators[idx2].id, self.cultivators[idx2].level.name());
                } else {
                    // idx2 胜利
                    let opponent_cultivation = self.cultivators[idx1].cultivation;
                    self.cultivators[idx2].absorb_cultivation(opponent_cultivation);
                    self.cultivators[idx1].alive = false;
                    defeated.push(idx1);
                 //    println!("战斗！修士{}({})击败修士{}({})", 
                 //        self.cultivators[idx2].id, self.cultivators[idx2].level.name(),
                 //        self.cultivators[idx1].id, self.cultivators[idx1].level.name());
                } 
            }
        }
    }

    fn yearly_cultivation(&mut self) {
        for cultivator in &mut self.cultivators {
            cultivator.yearly_cultivation();
        }
    }

    fn update_statistics(&mut self) {
        self.statistics.clear();
        for cultivator in &self.cultivators {
            if cultivator.alive {
                *self.statistics.entry(cultivator.level).or_insert(0) += 1;
            }
        }
    }

    fn calculate_average_courage_by_level(&self) -> HashMap<Level, (f64, u64)> {
        let mut level_courage_sum = HashMap::new();
        let mut level_count = HashMap::new();
        
        for cultivator in &self.cultivators {
            if cultivator.alive {
                let level = cultivator.level;
                *level_courage_sum.entry(level).or_insert(0.0) += cultivator.courage;
                *level_count.entry(level).or_insert(0) += 1;
            }
        }
        
        let mut averages = HashMap::new();
        for (level, sum) in level_courage_sum {
            let count = level_count[&level];
            let average = sum / count as f64;
            averages.insert(level, (average, count));
        }
        
        averages
    }

    fn calculate_overall_average_courage(&self) -> (f64, u64) {
        let alive_cultivators: Vec<_> = self.cultivators.iter()
            .filter(|c| c.alive)
            .collect();
        
        if alive_cultivators.is_empty() {
            return (0.0, 0);
        }
        
        let total_courage: f64 = alive_cultivators.iter()
            .map(|c| c.courage)
            .sum();
        
        let count = alive_cultivators.len() as u64;
        (total_courage / count as f64, count)
    }

    fn print_statistics(&self) {
        println!("\n=== 第{}年统计 ===", self.year);
        let mut total = 0;
        let courage_by_level = self.calculate_average_courage_by_level();
        
        for level in [Level::Foundation, Level::GoldenCore, Level::NascentSoul, 
                     Level::SpiritTransform, Level::VoidRefining, Level::BodyIntegration, Level::Mahayana] {
            let count = self.statistics.get(&level).unwrap_or(&0);
            if *count > 0 {
                let (avg_courage, _) = courage_by_level.get(&level).unwrap_or(&(0.0, 0));
                println!("{}期修士：{}人 (平均勇气值: {:.3})", 
                    level.name(), count, avg_courage);
                total += count;
            }
        }
        println!("总修士数：{}人", total);
        
        // 显示总体平均勇气值
        let (overall_avg_courage, total_count) = self.calculate_overall_average_courage();
        if total_count > 0 {
            println!("所有修士平均勇气值: {:.3}", overall_avg_courage);
        }
        
        // 显示一些高级修士的详细信息
        let high_level_cultivators: Vec<_> = self.cultivators.iter()
            .filter(|c| c.alive && c.level as u8 >= Level::NascentSoul as u8)
            .collect();
        
        if !high_level_cultivators.is_empty() {
            println!("\n高阶修士详情：");
            for cultivator in high_level_cultivators.iter().take(5) {
                println!("修士{}: {}期, 修为:{}, 年龄:{}, 剩余寿元:{}, 勇气值:{:.3}", 
                    cultivator.id, cultivator.level.name(), 
                    cultivator.cultivation, cultivator.age, cultivator.remaining_lifespan,
                    cultivator.courage);
            }
        }
    }

    fn simulate_year(&mut self) {
        self.year += 1;
        
        // 1. 新修士筑基
        self.add_new_cultivators();
        
        // 2. 所有修士修炼
        self.yearly_cultivation();
        
        // 3. 遭遇战斗
        self.encounter_and_battle();
        
        // 4. 清理死亡修士
        self.cultivators.retain(|c| c.alive);
        
        // 5. 更新统计
        self.update_statistics();
        
        // 6. 每10年显示一次统计
        if self.year % 10 == 0 {
            self.print_statistics();
        }
    }

    fn simulate(&mut self, years: u64) {
        println!("开始模拟修仙世界，模拟{}年...\n", years);
        
        for _ in 0..years {
            self.simulate_year();
        }
        
        println!("\n=== 模拟结束 ===");
        self.print_statistics();
    }
}

fn main() {
    let mut world = World::new();
    
    // 模拟10000年
    world.simulate(10000);
}
