#![cfg(test)]

mod tests {
    use super::*;
    use soroban_sdk::{Env, Address};

    #[test]
    fn happy_path() {
        let env = Env::default();
        let c = Address::generate(&env);
        let f = Address::generate(&env);

        SkillLock::create_escrow(env.clone(), 1, c.clone(), f.clone(), 50);
        SkillLock::release_payment(env.clone(), 1, c);

        assert_eq!(SkillLock::reputation_of(env, f), 1);
    }

    #[test]
    #[should_panic]
    fn duplicate_contract_rejected() {
        let env = Env::default();
        let c = Address::generate(&env);
        let f = Address::generate(&env);

        SkillLock::create_escrow(env.clone(), 1, c.clone(), f.clone(), 50);
        SkillLock::create_escrow(env.clone(), 1, c, f, 50);
    }

    #[test]
    fn state_verification() {
        let env = Env::default();
        let c = Address::generate(&env);
        let f = Address::generate(&env);

        SkillLock::create_escrow(env.clone(), 5, c, f, 100);

        assert!(
            env.storage()
                .persistent()
                .has(&DataKey::Escrow(5))
        );
    }

    #[test]
    #[should_panic]
    fn cannot_double_release() {
        let env = Env::default();
        let c = Address::generate(&env);
        let f = Address::generate(&env);

        SkillLock::create_escrow(env.clone(), 2, c.clone(), f, 25);
        SkillLock::release_payment(env.clone(), 2, c.clone());
        SkillLock::release_payment(env.clone(), 2, c);
    }

    #[test]
    fn reputation_accumulates() {
        let env = Env::default();
        let f = Address::generate(&env);

        SkillLock::mint_reputation(env.clone(), f.clone());
        SkillLock::mint_reputation(env.clone(), f.clone());

        assert_eq!(SkillLock::reputation_of(env, f), 2);
    }
}
