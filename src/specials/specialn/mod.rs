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

unsafe extern "C" fn diddy_specialn_pre(fighter: &mut L2CFighterCommon) -> L2CValue {
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
        (*FIGHTER_LOG_MASK_FLAG_ACTION_TRIGGER_ON | *FIGHTER_LOG_MASK_FLAG_ATTACK_KIND_SPECIAL_N) as u64, 
        0, 
        FIGHTER_POWER_UP_ATTACK_BIT_SPECIAL_N.into(), 
        0
    );
    0.into()
}

unsafe extern "C" fn diddy_specialn_main(fighter: &mut L2CFighterCommon) -> L2CValue {
    if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_start"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
    } else {
        MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_start"), 0.0, 1.0, false, 0.0, false, false);
        KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
        GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
    }
    fighter.sub_shift_status_main(L2CValue::Ptr(diddy_specialn_main_loop as *const () as _))
}

unsafe extern "C" fn diddy_specialn_main_loop(fighter: &mut L2CFighterCommon) -> L2CValue {
    let boma = fighter.module_accessor;
    let entry_id = WorkModule::get_int(boma, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
    
    let is_enable_cancel = CancelModule::is_enable_cancel(fighter.module_accessor);
    if is_enable_cancel {
        if fighter.sub_wait_ground_check_common(false.into()).get_bool()
        || fighter.sub_air_check_fall_common().get_bool() {
            return 1.into();
        }
    }
    
    let is_end = MotionModule::is_end(fighter.module_accessor);
    if is_end
    && (MotionModule::motion_kind(boma) == hash40("special_air_n_start")
    || MotionModule::motion_kind(boma) == hash40("special_n_start")) {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_air_n_loop"), 0.0, 1.0, false, 0.0, false, false);
        } else {
            MotionModule::change_motion(fighter.module_accessor, Hash40::new("special_n_loop"), 0.0, 1.0, false, 0.0, false, false);
        }
    }

    if MotionModule::motion_kind(boma) == hash40("special_air_n_start")
    || MotionModule::motion_kind(boma) == hash40("special_n_start") {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_start"), -1.0, 1.0, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_start"), -1.0, 1.0, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    
    if MotionModule::motion_kind(boma) == hash40("special_air_n_loop")
    || MotionModule::motion_kind(boma) == hash40("special_n_loop") {
        if fighter.global_table[0x16] != *SITUATION_KIND_GROUND {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_air_n_loop"), -1.0, 1.0, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_AIR_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_AIR));
        } else {
            MotionModule::change_motion_inherit_frame(fighter.module_accessor, Hash40::new("special_n_loop"), -1.0, 1.0, 0.0, false, false);
            KineticModule::change_kinetic(fighter.module_accessor, *FIGHTER_KINETIC_TYPE_GROUND_STOP);
            GroundModule::correct(fighter.module_accessor, GroundCorrectKind(*GROUND_CORRECT_KIND_GROUND_CLIFF_STOP));
        }
    }
    
    if fighter.global_table[0xe].get_i32() >= 10 {

        if ControlModule::get_stick_y(boma) > 0.809 
        && smashCD[entry_id] <= 0.0 {
            if state[entry_id] != "smash" {
                change[entry_id] = "smash";
                changing[entry_id] = true;
                totalTimer[entry_id] = 480.0; 
                r[entry_id] = 5.0;
                g[entry_id] = 0.0;
                b[entry_id] = 0.0;
                fighter.change_status(FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE.into(), false.into());
            }
        }
        if ControlModule::get_stick_y(boma) <= 0.809 
        && ControlModule::get_stick_y(boma) >= -0.309 
        && ControlModule::get_stick_x(boma) > 0.5 
        && busterCD[entry_id] <= 0.0 {
            if state[entry_id] != "buster" {
                change[entry_id] = "buster";
                changing[entry_id] = true;
                totalTimer[entry_id] = 600.0; 
                r[entry_id] = 3.0;
                g[entry_id] = 0.0;
                b[entry_id] = 3.0;
                fighter.change_status(FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE.into(), false.into());
            }
        }
        if ControlModule::get_stick_y(boma) <= 0.809 
        && ControlModule::get_stick_y(boma) >= -0.309 
        && ControlModule::get_stick_x(boma) < -0.5 
        && jumpCD[entry_id] <= 0.0 { 
            if state[entry_id] != "jump" {
                change[entry_id] = "jump";
                changing[entry_id] = true;
                totalTimer[entry_id] = 360.0; 
                r[entry_id] = 0.0;
                g[entry_id] = 5.0;
                b[entry_id] = 0.0;
                fighter.change_status(FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE.into(), false.into());
            }
        }
        if ControlModule::get_stick_y(boma) < -0.309 
        && ControlModule::get_stick_x(boma) > 0.5 
        && shieldCD[entry_id] <= 0.0 {
            if state[entry_id] != "shield" {
                change[entry_id] = "shield";
                changing[entry_id] = true;
                totalTimer[entry_id] = 360.0; 
                r[entry_id] = 5.0;
                g[entry_id] = 5.0;
                b[entry_id] = 0.0;
                fighter.change_status(FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE.into(), false.into());
            }
        }
        if ControlModule::get_stick_y(boma) < -0.309 
        && ControlModule::get_stick_x(boma) < -0.5 
        && speedCD[entry_id] <= 0.0 {
            if state[entry_id] != "speed" {
                change[entry_id] = "speed";
                changing[entry_id] = true;
                totalTimer[entry_id] = 480.0; 
                r[entry_id] = 0.0;
                g[entry_id] = 4.0;
                b[entry_id] = 4.0;
                fighter.change_status(FIGHTER_DIDDY_STATUS_KIND_SPECIAL_N_CHARGE.into(), false.into());
            }
        }
    }

    0.into()
}

unsafe extern "C" fn diddy_specialn_end(fighter: &mut L2CFighterCommon) -> L2CValue {
    0.into()
}

pub fn install() {
    Agent::new("diddy")
        .status(Pre, *FIGHTER_STATUS_KIND_SPECIAL_N, diddy_specialn_pre)
        .status(Main, *FIGHTER_STATUS_KIND_SPECIAL_N, diddy_specialn_main)
        .status(End, *FIGHTER_STATUS_KIND_SPECIAL_N, diddy_specialn_end)
        .install();
}
