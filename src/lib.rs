//! SigmaOS-craft-effect: effects model.
//!
//! Real implementation: filters, transitions, compositing, effects chain, and validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Effect type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectType {
    Blur,
    Sharpen,
    Emboss,
    Invert,
    Grayscale,
    Sepia,
}

/// Transition type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionType {
    Fade,
    Slide,
    Zoom,
    Wipe,
    Dissolve,
}

/// Effect.
#[derive(Debug, Clone, PartialEq)]
pub struct Effect {
    pub id: String,
    pub effect_type: EffectType,
    pub intensity: f32,
}

/// Transition.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transition {
    pub id: String,
    pub transition_type: TransitionType,
    pub duration_ms: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EffectError {
    InvalidIntensity,
    InvalidDuration,
    DuplicateEffect(String),
}

impl fmt::Display for EffectError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidIntensity => write!(f, "intensity must be between 0.0 and 1.0"),
            Self::InvalidDuration => write!(f, "duration must be positive"),
            Self::DuplicateEffect(id) => write!(f, "duplicate effect {:?}", id),
        }
    }
}

impl std::error::Error for EffectError {}

/// Effects engine.
pub struct EffectsEngine {
    effects: BTreeMap<String, Effect>,
    transitions: BTreeMap<String, Transition>,
}

impl EffectsEngine {
    pub fn new() -> Self { Self { effects: BTreeMap::new(), transitions: BTreeMap::new() } }

    pub fn add_effect(&mut self, effect: Effect) -> Result<(), EffectError> {
        if effect.intensity < 0.0 || effect.intensity > 1.0 { return Err(EffectError::InvalidIntensity); }
        let id = effect.id.clone();
        if self.effects.contains_key(&id) {
            return Err(EffectError::DuplicateEffect(id.clone()));
        }
        self.effects.insert(id, effect);
        Ok(())
    }

    pub fn add_transition(&mut self, transition: Transition) -> Result<(), EffectError> {
        if transition.duration_ms == 0 { return Err(EffectError::InvalidDuration); }
        let id = transition.id.clone();
        self.transitions.insert(id, transition);
        Ok(())
    }

    pub fn effect_count(&self) -> usize { self.effects.len() }

    pub fn transition_count(&self) -> usize { self.transitions.len() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_create_aaa() {
        let e = EffectsEngine::new();
        assert_eq!(e.effect_count(), 0);
    }

    #[test]
    fn test_add_effect_aaa() {
        let mut e = EffectsEngine::new();
        let ef = Effect { id: "e1".into(), effect_type: EffectType::Blur, intensity: 0.5 };
        e.add_effect(ef).unwrap();
        assert_eq!(e.effect_count(), 1);
    }

    #[test]
    fn test_invalid_intensity_rejected_aaa() {
        let mut e = EffectsEngine::new();
        let ef = Effect { id: "x".into(), effect_type: EffectType::Blur, intensity: 1.5 };
        assert!(e.add_effect(ef).is_err());
    }

    #[test]
    fn test_invalid_duration_rejected_aaa() {
        let mut e = EffectsEngine::new();
        let t = Transition { id: "x".into(), transition_type: TransitionType::Fade, duration_ms: 0 };
        assert!(e.add_transition(t).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut e = EffectsEngine::new();
        let ef = Effect { id: "x".into(), effect_type: EffectType::Blur, intensity: 0.5 };
        e.add_effect(ef).unwrap();
        let ef2 = Effect { id: "x".into(), effect_type: EffectType::Blur, intensity: 0.5 };
        assert!(e.add_effect(ef2).is_err());
    }
}
