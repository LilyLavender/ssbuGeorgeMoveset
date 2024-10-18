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

unsafe extern "C" fn diddy_specialnaction_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
    StatusModule::init_settings(
        fighter.module_accessor, 
        smash::app::SituationKind(*SITUATION_KIND_NONE),  
        *FIGHTER_KINETIC_TYPE_UNIQ,
        GROUND_CORRECT_KIND_KEEP.into(), 
        smash::app::GroundCliffCheckKind(*GROUND_CLIFF_CHECK_KIND_NONE), 
        true, 
        FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLAG.into(), 
        *FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_INT, 
        FIGHTER_STATUS_WORK_KEEP_FLAG_NONE_FLOAT.into(), 
        0
    );
    FighterStatusModuleImpl::set_fighter_status_data(
        fighter.module_accessor, 
        false, 
        *FIGHTER_TREADED_KIND_NO_REAC, 
        false, 
        false, 
        false, 
        *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N as u64,
        0, 
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N.into(), 
        0
    );
    0.into()
}

unsafe extern "C" fn diddy_specialnaction_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;

    let sum_speed_x = KineticModule::get_sum_speed_x(boma, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN);
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        
        fighter.agent.clear_lua_stack();
        fighter.agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(sum_speed_x));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    } else {
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        
        fighter.agent.clear_lua_stack();
        fighter.agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_STOP as u64));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(sum_speed_x));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        sv_kinetic_energy::set_speed(fighter.lua_state_agent);
    }
    
    KineticModule::enable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_STOP);
    
    fighter.agent.clear_lua_stack();
    fighter.agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_CONTROL as u64));
    fighter.agent.push_lua_stack(&mut L2CValue::new_int(*ENERGY_CONTROLLER_RESET_TYPE_FREE as u64));
    fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
    sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
    
    KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_CONTROL);
    
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        fighter.agent.clear_lua_stack();
        fighter.agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_MOTION as u64));
        fighter.agent.push_lua_stack(&mut L2CValue::new_int(*ENERGY_MOTION_RESET_TYPE_AIR_TRANS as u64));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    } else {
        fighter.agent.clear_lua_stack();
        fighter.agent.push_lua_stack(&mut L2CValue::new_int(*FIGHTER_KINETIC_ENERGY_ID_MOTION as u64));
        fighter.agent.push_lua_stack(&mut L2CValue::new_int(*ENERGY_MOTION_RESET_TYPE_GROUND_TRANS as u64));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        fighter.agent.push_lua_stack(&mut L2CValue::new_num(0.0));
        sv_kinetic_energy::reset_energy(fighter.lua_state_agent);
        
        KineticModule::unable_energy(boma, *FIGHTER_KINETIC_ENERGY_ID_MOTION);
    }
    
    let special_n_type = state[entry_id];
    let mut motion_kind_ground = "";
    let mut motion_kind_air = "";
    
    if special_n_type == "jump" {
        motion_kind_ground = "special_n_jump";
        motion_kind_air = "special_air_n_jump";
    } else if special_n_type == "speed" {
        motion_kind_ground = "special_n_speed";
        motion_kind_air = "special_air_n_speed";
    } else if special_n_type == "shield" {
        motion_kind_ground = "special_n_shield";
        motion_kind_air = "special_air_n_shield";
    } else if special_n_type == "buster" {
        motion_kind_ground = "special_n_buster";
        motion_kind_air = "special_air_n_buster";
    } else if special_n_type == "smash" {
        motion_kind_ground = "special_n_smash";
        motion_kind_air = "special_air_n_smash";
    } 
    
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(boma, Hash40::new(motion_kind_air), 0.0, 1.0, false, 0.0, false, false);
    } else {
        MotionModule::change_motion(boma, Hash40::new(motion_kind_ground), 0.0, 1.0, false, 0.0, false, false);
    }
    WorkModule::unable_transition_term_group_ex(boma, *FIGHTER_STATUS_TRANSITION_TERM_ID_CONT_JUMP_SQUAT);

    fighter.sub_shift_status_main(L2CValue::Ptr(diddy_specialnaction_main_loop as *const () as _))
}

unsafe extern "C" fn diddy_specialnaction_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    if MotionModule::is_end(boma) {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), true.into());
        } else {
            fighter.change_status(FIGHTER_STATUS_KIND_WAIT.into(), true.into());
        }
    }

    let special_n_type = state[entry_id];
    let mut motion_kind_ground = "";
    let mut motion_kind_air = "";
    
    if special_n_type == "jump" {
        motion_kind_ground = "special_n_jump";
        motion_kind_air = "special_air_n_jump";
    } else if special_n_type == "speed" {
        motion_kind_ground = "special_n_speed";
        motion_kind_air = "special_air_n_speed";
    } else if special_n_type == "shield" {
        motion_kind_ground = "special_n_shield";
        motion_kind_air = "special_air_n_shield";
    } else if special_n_type == "buster" {
        motion_kind_ground = "special_n_buster";
        motion_kind_air = "special_air_n_buster";
    } else if special_n_type == "smash" {
        motion_kind_ground = "special_n_smash";
        motion_kind_air = "special_air_n_smash";
    } 

    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        MotionModule::change_motion_inherit_frame(boma, Hash40::new(motion_kind_air), -1.0, 1.0, 0.0, false, false);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    } else {
        MotionModule::change_motion_inherit_frame(boma, Hash40::new(motion_kind_ground), -1.0, 1.0, 0.0, false, false);
        KineticModule::change_kinetic(boma, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(boma, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }

    return 0.into();
}

unsafe extern "C" fn diddy_specialnaction_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0xb] != *FIGHTER_STATUS_KIND_ITEM_SWING_S4_START {
        return 0.into();
    }
    
    let have_item_kind = ItemModule::get_have_item_kind(fighter.module_accessor, 0);
    if have_item_kind == *ITEM_KIND_HOMERUNBAT {
        HitModule::set_whole(fighter.module_accessor, smash::app::HitStatus(*HIT_STATUS_NORMAL), 0);
    }
    
    return 0.into();
}

pub fn install() {
    Agent::new("diddy")
        .status(Pre, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE, diddy_specialnaction_pre)
        .status(Main, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE, diddy_specialnaction_main)
        .status(End, *FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE, diddy_specialnaction_end)
        .install();
}
