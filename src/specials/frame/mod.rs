use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*},
    super::*
};

unsafe extern "C" fn diddy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let boma = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
        let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
        let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
        if state[entry_id] == "smash" { //smash
            r[entry_id] = 5.0;
            g[entry_id] = 0.0;
            b[entry_id] = 0.0;
            AttackModule::set_power_up(boma, 0.3);
            AttackModule::set_reaction_mul(boma, 1.25);
            DamageModule::set_damage_mul(boma, 1.0);
            DamageModule::set_reaction_mul(boma, 1.2);
            ShieldModule::set_attack_mul(boma, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
        }
        if state[entry_id] == "buster" { //buster
            r[entry_id] = 3.0;
            g[entry_id] = 0.0;
            b[entry_id] = 3.0;
            AttackModule::set_power_up(boma, 1.4);
            AttackModule::set_reaction_mul(boma, 0.65);
            DamageModule::set_damage_mul(boma, 1.3);
            DamageModule::set_reaction_mul(boma, 1.0);
            ShieldModule::set_attack_mul(boma, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            ShieldModule::set_hit_stop_mul(boma, 1.0);
        }
        if state[entry_id] == "jump" { //jump
            if MotionModule::motion_kind(boma) != smash::hash40("jump_f_mini") && MotionModule::motion_kind(boma) != smash::hash40("jump_b_mini") {
                r[entry_id] = 0.0;
                g[entry_id] = 5.0;
                b[entry_id] = 0.0;
                if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP_AERIAL || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                    if MotionModule::frame(boma) < 1.0  {
                        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 1.7, z: 0.0} as *const Vector3f);
                    }
                }
                if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP {
                    if MotionModule::frame(boma) < 5.0 {
                        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: 1.7, z: 0.0} as *const Vector3f);
                    }
                }
                if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_FALL {
                    KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -0.12, z: 0.0} as *const Vector3f);
                }
                AttackModule::set_power_up(boma, 1.0);
                AttackModule::set_reaction_mul(boma, 1.0);
                DamageModule::set_damage_mul(boma, 1.3);
                DamageModule::set_reaction_mul(boma, 1.0);
                ShieldModule::set_attack_mul(boma, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
                ShieldModule::set_hit_stop_mul(boma, 1.0);
            }
        }
        if state[entry_id] == "shield" { //shield
            if MotionModule::motion_kind(boma) != smash::hash40("jump_f_mini") && MotionModule::motion_kind(boma) != smash::hash40("jump_b_mini") {
                r[entry_id] = 5.0;
                g[entry_id] = 5.0;
                b[entry_id] = 0.0;
                if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP_AERIAL || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                    if MotionModule::frame(boma) < 1.0 {
                        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -0.16, z: 0.0} as *const Vector3f);
                    }
                }
                if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP {
                    if MotionModule::frame(boma) < 5.0 {
                        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -0.84, z: 0.0} as *const Vector3f);
                    }
                }
                if StatusModule::situation_kind(boma) == *SITUATION_KIND_AIR && StatusModule::status_kind(boma) != *FIGHTER_STATUS_KIND_SPECIAL_HI {
                    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.6);
                }
                if StatusModule::situation_kind(boma) == *SITUATION_KIND_GROUND {
                    smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 0.6);
                    sv_kinetic_energy!(mul_x_speed_max, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.6);
                    sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.6);
                }
                AttackModule::set_power_up(boma, 0.5);
                AttackModule::set_reaction_mul(boma, 0.8);
                DamageModule::set_damage_mul(boma, 0.5);
                DamageModule::set_reaction_mul(boma, 0.6);
                ShieldModule::set_attack_mul(boma, 0.5, *FIGHTER_SHIELD_KIND_GUARD);
                ShieldModule::set_hit_stop_mul(boma, 0.8);
            }
        }
        if state[entry_id] == "speed" { //speed
            if MotionModule::motion_kind(boma) != smash::hash40("jump_f_mini") && MotionModule::motion_kind(boma) != smash::hash40("jump_b_mini") {
                r[entry_id] = 0.0;
                g[entry_id] = 4.0;
                b[entry_id] = 4.0;
                if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP_AERIAL || StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_SPECIAL_HI {
                    if MotionModule::frame(boma) < 1.0 {
                        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -0.2, z: 0.0} as *const Vector3f);
                    }
                }
                if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_JUMP {
                    if MotionModule::frame(boma) < 5.0 {
                        KineticModule::add_speed(boma, &Vector3f{x: 0.0, y: -1.0, z: 0.0} as *const Vector3f);
                    }
                }
                
                sv_kinetic_energy!(set_accel_y_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.25);
                sv_kinetic_energy!(set_accel_x_mul, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.25);
                sv_kinetic_energy!(set_limit_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 2.0);
                sv_kinetic_energy!(set_stable_speed, fighter, FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.5);
                smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 2.0);
                AttackModule::set_power_up(boma, 0.7);
                AttackModule::set_reaction_mul(boma, 1.0);
                DamageModule::set_damage_mul(boma, 1.0);
                DamageModule::set_reaction_mul(boma, 1.0);
                ShieldModule::set_attack_mul(boma, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
                ShieldModule::set_hit_stop_mul(boma, 1.0);
            }
        }
        if state[entry_id] == "none" { //none
            r[entry_id] = 0.0;
            g[entry_id] = 0.0;
            b[entry_id] = 0.0;
            AttackModule::set_power_up(boma, 1.0);
            AttackModule::set_reaction_mul(boma, 1.0);
            DamageModule::set_damage_mul(boma, 1.0);
            DamageModule::set_reaction_mul(boma, 1.0);
            ShieldModule::set_attack_mul(boma, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
            ShieldModule::set_hit_stop_mul(boma, 1.0);
        }
        if StatusModule::status_kind(boma) == *FIGHTER_STATUS_KIND_DEAD || sv_information::is_ready_go() == false {
            smashCD[entry_id] = 0.0;
            busterCD[entry_id] = 0.0;
            speedCD[entry_id] = 0.0;
            shieldCD[entry_id] = 0.0;
            jumpCD[entry_id] = 0.0;
            totalTimer[entry_id] = 0.0;
            change[entry_id] = "none";
            changing[entry_id] = true;
        }
        if timer[entry_id] <= 0.0 && state[entry_id] != "none" {
            change[entry_id] = "none";
            totalTimer[entry_id] = 0.0;
            r[entry_id] = 2.5;
            b[entry_id] = 2.5;
            g[entry_id] = 2.5;
            changing[entry_id] = true;
        }
        if timer[entry_id] > 0.0 {
            timer[entry_id] = timer[entry_id] - 1.0;
        }
        if smashCD[entry_id] > 0.0 {
            smashCD[entry_id] = smashCD[entry_id] - 1.0;
        }
        if busterCD[entry_id] > 0.0 {
            busterCD[entry_id] = busterCD[entry_id] - 1.0;
        }
        if jumpCD[entry_id] > 0.0 {
            jumpCD[entry_id] = jumpCD[entry_id] - 1.0;
        }
        if shieldCD[entry_id] > 0.0 {
            shieldCD[entry_id] = shieldCD[entry_id] - 1.0;
        }
        if speedCD[entry_id] > 0.0 {
            speedCD[entry_id] = speedCD[entry_id] - 1.0;
        }
        if change[entry_id] == state[entry_id] {
            change[entry_id] = "none";
            changing[entry_id] = false;
        }
        if state[entry_id] != "none" && timer[entry_id] <= 0.0 || changing[entry_id] == true {
            if state[entry_id] == "smash" {
                smashCD[entry_id] = 960.0;
            }
            if state[entry_id] == "buster" {
                busterCD[entry_id] = 840.0;
            }
            if state[entry_id] == "jump" {
                jumpCD[entry_id] = 1080.0; 
            }
            if state[entry_id] == "shield" {
                shieldCD[entry_id] = 1080.0;
            }
            if state[entry_id] == "speed" {
                speedCD[entry_id] = 960.0;
            }
            timer[entry_id] = totalTimer[entry_id]; 
            state[entry_id] = change[entry_id]; 
            changing[entry_id] = false; 
        }
        effect[entry_id] += 1;
        if effect[entry_id] == 10 {
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, r[entry_id], g[entry_id], b[entry_id]);
        }
        if effect[entry_id] >= 20 {
            macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
            macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
            macros::LAST_EFFECT_SET_COLOR(fighter, r[entry_id], g[entry_id], b[entry_id]);
            effect[entry_id] = 0;
        }
    }
}

pub fn install() {
    Agent::new("diddy")
        .on_line(Main, diddy_frame)
        .install();
}
