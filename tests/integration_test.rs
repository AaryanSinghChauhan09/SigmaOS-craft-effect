use sigmaos_craft_effect::{EffectsEngine, Effect, EffectType};

#[test]
fn effects_engine_integration_aaa() {
    let mut e = EffectsEngine::new();
    e.add_effect(Effect { id: "e1".into(), effect_type: EffectType::Blur, intensity: 0.5 }).unwrap();
    assert_eq!(e.effect_count(), 1);
}
