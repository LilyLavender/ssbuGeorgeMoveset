use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
        hash40
    },
    smash_script::*,
    smashline::{*, Priority::*}
};

unsafe extern "C" fn diddy_game_specialhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 83, 36, 0, 132, 6.0, 0.0, 6.0, 7.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 83, 36, 0, 132, 6.0, 0.0, 6.0, -7.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 0, 10.0, false);
        AttackModule::set_add_reaction_frame(agent.module_accessor, 1, 10.0, false);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 6.0, 361, 80, 0, 60, 5.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_DECIDE_DIRECTION);
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_PARACHUTE_OPEN);
    }
}

unsafe extern "C" fn diddy_game_specialairhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 3.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("top"), 3.0, 75, 70, 0, 100, 6.0, 0.0, 6.0, 7.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 3.0, 75, 70, 0, 100, 6.0, 0.0, 6.0, -7.5, None, None, None, 1.5, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_POS, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_OBJECT);
    }
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 0, 0, Hash40::new("waist"), 6.0, 361, 80, 0, 60, 5.5, 1.0, 0.0, 0.0, None, None, None, 1.0, 1.0, *ATTACK_SETOFF_KIND_ON, *ATTACK_LR_CHECK_F, false, 0, 0.0, 0, false, false, false, false, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_M, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_BODY);
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_DECIDE_DIRECTION);
    }
    frame(agent.lua_state_agent, 10.0);
    macros::FT_MOTION_RATE(agent, 0.8);
    frame(agent.lua_state_agent, 11.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_LANDING_ENABLE);
    }
    frame(agent.lua_state_agent, 14.0);
    if macros::is_excute(agent) {
        notify_event_msc_cmd!(agent, Hash40::new_raw(0x2127e37c07), *GROUND_CLIFF_CHECK_KIND_ALWAYS);
    }
    frame(agent.lua_state_agent, 21.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    macros::FT_MOTION_RATE(agent, 1.0);
    frame(agent.lua_state_agent, 33.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_GAMEWATCH_STATUS_SPECIAL_HI_FLAG_PARACHUTE_OPEN);
    }
}

unsafe extern "C" fn diddy_effect_specialnbuster(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_equip_end"), Hash40::new("haver"), 0, 0, 0, 0, 0, 0, 0.7, true, 0.608, 0.392, 1);
        sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
    }
}

unsafe extern "C" fn diddy_effect_specialnjump(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_equip_end"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.7, true, 0.541, 1, 0.51);
        sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_equip_end"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, true, 0.541, 1, 0.51);
        sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
    }
}

unsafe extern "C" fn diddy_effect_specialnshield(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_equip_end"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 0.7, true, 1, 0.984, 0.733);
        sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
    }
}

unsafe extern "C" fn diddy_effect_specialnsmash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_equip_end"), Hash40::new("handr"), 0, 0, 0, 0, 0, 0, 0.7, true, 1, 0.51, 0.588);
        sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_equip_end"), Hash40::new("handl"), 0, 0, 0, 0, 0, 0, 0.7, true, 1, 0.51, 0.588);
        sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
    }
}

unsafe extern "C" fn diddy_effect_specialnspeed(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    if macros::is_excute(agent) {
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_equip_end"), Hash40::new("footr"), 0, 0, 0, 0, 0, 0, 0.7, true, 0.392, 0.549, 1);
        sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
        agent.clear_lua_stack();
        lua_args!(agent, Hash40::new("sys_equip_end"), Hash40::new("footl"), 0, 0, 0, 0, 0, 0, 0.7, true, 0.392, 0.549, 1);
        sv_animcmd::EFFECT_FOLLOW_COLOR(agent.lua_state_agent);
    }
}

pub fn install() {
    Agent::new("diddy")
        .game_acmd("game_specialhi", diddy_game_specialhi, Default)
        .game_acmd("game_specialairhi", diddy_game_specialairhi, Default)
        .effect_acmd("effect_specialnbuster", diddy_effect_specialnbuster, Default)
        .effect_acmd("effect_specialnjump", diddy_effect_specialnjump, Default)
        .effect_acmd("effect_specialnshield", diddy_effect_specialnshield, Default)
        .effect_acmd("effect_specialnsmash", diddy_effect_specialnsmash, Default)
        .effect_acmd("effect_specialnspeed", diddy_effect_specialnspeed, Default)
        .install();
}
