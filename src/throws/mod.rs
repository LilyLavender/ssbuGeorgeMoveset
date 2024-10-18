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

unsafe extern "C" fn diddy_game_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 11.0);
	if macros::is_excute(agent) {
		GrabModule::set_rebound(agent.module_accessor, true);
	}
	frame(agent.lua_state_agent, 15.0);
	if macros::is_excute(agent) {
		macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 19.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 23.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 27.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 31.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 35.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	macros::game_CaptureCutCommon(agent);
	frame(agent.lua_state_agent, 23.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		GrabModule::set_rebound(agent.module_accessor, false);
	}

}

unsafe extern "C" fn diddy_game_catchdash(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 16.0);
	if macros::is_excute(agent) {	
		GrabModule::set_rebound(agent.module_accessor, true);
	}
	frame(agent.lua_state_agent, 17.0);
		if macros::is_excute(agent) {	
		macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 7.5, 13.0, Some(0.0), Some(7.5), Some(5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 19.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 23.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 27.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 31.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, 35.0, Some(0.0), Some(7.5), Some(15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	macros::game_CaptureCutCommon(agent);	
	frame(agent.lua_state_agent, 25.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		GrabModule::set_rebound(agent.module_accessor, false);
	}
}

unsafe extern "C" fn diddy_game_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 17.0);
	if macros::is_excute(agent) {	
		GrabModule::set_rebound(agent.module_accessor, true);
	}
	frame(agent.lua_state_agent, 18.0);
	if macros::is_excute(agent) {	
		macros::CATCH(agent, 0, Hash40::new("top"), 3.0, 0.0, 7.5, -13.0, Some(0.0), Some(7.5), Some(-5.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, -19.0, Some(0.0), Some(7.5), Some(-15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, -23.0, Some(0.0), Some(7.5), Some(-15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, -27.0, Some(0.0), Some(7.5), Some(-15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, -31.0, Some(0.0), Some(7.5), Some(-15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	wait(agent.lua_state_agent, 1.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		macros::CATCH(agent, 0, Hash40::new("top"), 4.0, 0.0, 7.5, -35.0, Some(0.0), Some(7.5), Some(-15.0), *FIGHTER_STATUS_KIND_CAPTURE_PULLED, *COLLISION_SITUATION_MASK_GA);
	}
	macros::game_CaptureCutCommon(agent);
	frame(agent.lua_state_agent, 26.0);
	if macros::is_excute(agent) {	
		grab!(agent, *MA_MSC_CMD_GRAB_CLEAR_ALL);
		GrabModule::set_rebound(agent.module_accessor, false);
	}
}

unsafe extern "C" fn diddy_effect_catch(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 15.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, 13, 0, 0, 0, 0.3, true);
	}
	frame(agent.lua_state_agent, 16.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, 19, 0, 0, 0, 0.2, true);
	}
	frame(agent.lua_state_agent, 17.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, 23, 0, 0, 0, 0.2, true);
	}
	frame(agent.lua_state_agent, 18.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, 27, 0, 0, 0, 0.2, true);
	}
	frame(agent.lua_state_agent, 19.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, 31, 0, 0, 0, 0.2, true);
	}
	frame(agent.lua_state_agent, 20.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, 35, 0, 0, 0, 0.2, true);
	}
}

unsafe extern "C" fn diddy_effect_catchturn(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 18.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, -13, 0, 0, 0, 0.3, true);
	}
	frame(agent.lua_state_agent, 19.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, -19, 0, 0, 0, 0.2, true);
	}
	frame(agent.lua_state_agent, 20.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, -23, 0, 0, 0, 0.2, true);
	}
	frame(agent.lua_state_agent, 21.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, -27, 0, 0, 0, 0.2, true);
	}
	frame(agent.lua_state_agent, 22.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, -31, 0, 0, 0, 0.2, true);
	}
	frame(agent.lua_state_agent, 23.0);
	if macros::is_excute(agent) {	
		macros::EFFECT_FOLLOW(agent, Hash40::new("sys_hit_elec"), Hash40::new("top"), 0, 7.5, -35, 0, 0, 0, 0.2, true);
	}
}

unsafe extern "C" fn diddy_game_throwf(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 5.0, 75, 125, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        WorkModule::on_flag(agent.module_accessor, *FIGHTER_STATUS_THROW_FLAG_START_AIR);
    }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 7, 0);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.8);
        lua_bind::FighterCutInManager::set_throw_finish_offset(singletons::FighterCutInManager(), Vector3f{x: 0.0, y: 0.0, z: 0.0});
    }
    frame(agent.lua_state_agent, 45.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn diddy_effect_throwf(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 9.0);
    if macros::is_excute(agent) {
        macros::FOOT_EFFECT(agent, Hash40::new("sys_turn_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
    frame(agent.lua_state_agent, 17.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 0, 2, 0, 0, 270, 0.85, true, 0.6);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(agent.lua_state_agent, 24.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_ALPHA(agent, Hash40::new("sys_spin_wind"), Hash40::new("rot"), 0, 0, 2, 0, 0, 270, 0.9, true, 0.6);
        macros::LAST_EFFECT_SET_RATE(agent, 2);
    }
    frame(agent.lua_state_agent, 34.0);
    if macros::is_excute(agent) {
        macros::EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 14, 6, -90, 0, 0, 0.8, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 36.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("trans"), 4, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::EFFECT_FOLLOW_NO_STOP_FLIP(agent, Hash40::new("sys_attack_speedline"), Hash40::new("sys_attack_speedline"), Hash40::new("top"), 0, 11, 6, -90, 0, 0, 0.6, true, *EF_FLIP_YZ);
    }
    frame(agent.lua_state_agent, 39.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_down_smoke"), Hash40::new("top"), 4, 0, 0, 0, 0, 0, 0.8, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn diddy_game_throwb(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 0.0, 47, 340, 0, 57, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::ATTACK(agent, 1, 0, Hash40::new("top"), 14.0, 32, 80, 0, 60, 7.0, 0.0, 2.7, 2.6, Some(0.0), Some(2.7), Some(-3.0), 1.3, 1.0, *ATTACK_SETOFF_KIND_OFF, *ATTACK_LR_CHECK_B, false, 0, 0.0, 0, false, false, false, true, true, *COLLISION_SITUATION_MASK_GA, *COLLISION_CATEGORY_MASK_ALL, *COLLISION_PART_MASK_ALL, false, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_L, *COLLISION_SOUND_ATTR_PUNCH, *ATTACK_REGION_PUNCH);
        AttackModule::set_catch_only_all(agent.module_accessor, true, false);
        macros::CHECK_FINISH_CAMERA(agent, -4, 5);
    }
    frame(agent.lua_state_agent, 29.0);
    if macros::is_excute(agent) {
        AttackModule::clear_all(agent.module_accessor);
    }
    frame(agent.lua_state_agent, 30.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 50.0);
    if macros::is_excute(agent) {
        macros::REVERSE_LR(agent);
    }
}

unsafe extern "C" fn diddy_effect_throwb(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 22.0);
    if macros::is_excute(agent) {
        macros::LANDING_EFFECT(agent, Hash40::new("sys_atk_smoke"), Hash40::new("top"), 0, 0, 0, 0, 180, 0, 1, 0, 0, 0, 0, 0, 0, false);
        macros::LAST_EFFECT_SET_RATE(agent, 0.7);
    }
    frame(agent.lua_state_agent, 27.0);
    frame(agent.lua_state_agent, 28.0);
    if macros::is_excute(agent) {
        macros::EFFECT(agent, Hash40::new("sys_crown"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, false);
        macros::LANDING_EFFECT(agent, Hash40::new("null"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
    }
}

unsafe extern "C" fn diddy_game_throwhi(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 12.0, 90, 65, 0, 72, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 3.0, 361, 100, 0, 40, 0.0, 1.0, *ATTACK_LR_CHECK_F, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 42.0);
    if macros::is_excute(agent) {
        macros::CHECK_FINISH_CAMERA(agent, 0, 27);
        lua_bind::FighterCutInManager::set_throw_finish_zoom_rate(singletons::FighterCutInManager(), 1.2);
    }
    frame(agent.lua_state_agent, 43.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
}

unsafe extern "C" fn diddy_effect_throwhi(agent: &mut L2CAgentBase) {
    frame(agent.lua_state_agent, 5.0);
    for _ in 0..10 {
    if macros::is_excute(agent) {
        macros::EFFECT_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("throw"), 2, 0, 1, 0, 0, 0, 0.5, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
        macros::LAST_EFFECT_SET_COLOR(agent, 1.5, 0.5, 1.5);
        macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    }
    wait(agent.lua_state_agent, 3.0);
}
frame(agent.lua_state_agent, 42.0);
if macros::is_excute(agent) {
    macros::EFFECT_FLIP(agent, Hash40::new("sys_flash"), Hash40::new("sys_flash"), Hash40::new("throw"), 2, 0, 1, 0, 0, 0, 0.9, 0, 0, 0, 0, 0, 0, true, *EF_FLIP_YZ);
    macros::LAST_EFFECT_SET_COLOR(agent, 1.5, 0.5, 1.5);
    macros::LAST_EFFECT_SET_RATE(agent, 1.3);
    macros::LANDING_EFFECT(agent, Hash40::new("sys_landing_smoke"), Hash40::new("top"), 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, false);
}
}

unsafe extern "C" fn diddy_game_throwlw(agent: &mut L2CAgentBase) {
    if macros::is_excute(agent) {
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, 0, 9.0, 270, 0, 0, 0, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_lay"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
        macros::ATTACK_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_CATCH, 0, 2.0, 361, 100, 0, 60, 0.0, 1.0, *ATTACK_LR_CHECK_B, 0.0, true, Hash40::new("collision_attr_normal"), *ATTACK_SOUND_LEVEL_S, *COLLISION_SOUND_ATTR_NONE, *ATTACK_REGION_THROW);
    }
    frame(agent.lua_state_agent, 35.0);
    if macros::is_excute(agent) {
        let target = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_OBJECT);
        let target_group = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_GROUP);
        let target_no = WorkModule::get_int64(agent.module_accessor, *FIGHTER_STATUS_THROW_WORK_INT_TARGET_HIT_NO);
        macros::ATK_HIT_ABS(agent, *FIGHTER_ATTACK_ABSOLUTE_KIND_THROW, Hash40::new("throw"), target, target_group, target_no);
    }
    frame(agent.lua_state_agent, 36.0);
    macros::FT_MOTION_RATE(agent, 0.5);
}

unsafe extern "C" fn diddy_effect_throwlw(agent: &mut L2CAgentBase) {}

pub fn install() {
    Agent::new("diddy")
        .game_acmd("game_catch", diddy_game_catch, Default)
        .game_acmd("game_catchdash", diddy_game_catchdash, Default)
        .game_acmd("game_catchturn", diddy_game_catchturn, Default)
        .effect_acmd("effect_catch", diddy_effect_catch, Default)
        .effect_acmd("effect_catchdash", diddy_effect_catch, Default)
        .effect_acmd("effect_catchturn", diddy_effect_catchturn, Default)
        .game_acmd("game_throwf", diddy_game_throwf, Default)
        .effect_acmd("effect_throwf", diddy_effect_throwf, Default)
        .game_acmd("game_throwb", diddy_game_throwb, Default)
        .effect_acmd("effect_throwb", diddy_effect_throwb, Default)
        .game_acmd("game_throwhi", diddy_game_throwhi, Default)
        .effect_acmd("effect_throwhi", diddy_effect_throwhi, Default)
        .game_acmd("game_throwlw", diddy_game_throwlw, Default)
        .effect_acmd("effect_throwlw", diddy_effect_throwlw, Default)
        .install();
}
