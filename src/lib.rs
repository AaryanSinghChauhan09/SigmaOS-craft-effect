#[derive(Debug, Clone)]
pub struct Effect { pub name: String, pub intensity: f32 }

#[derive(Debug, Default)]
pub struct EffectChain { effects: Vec<Effect> }
impl EffectChain {
    pub fn new() -> Self { Self::default() }
    pub fn push(&mut self, name: impl Into<String>, intensity: f32) {
        self.effects.push(Effect { name: name.into(), intensity });
    }
    pub fn count(&self) -> usize { self.effects.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_effects() {
        let mut chain = EffectChain::new();
        chain.push("blur", 0.5);
        assert_eq!(chain.count(), 1);
    }
}
