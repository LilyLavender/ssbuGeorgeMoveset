use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
    },
    smash_script::*,
    smashline::*
};

static mut state: [&str; 8] = ["none"; 8];
static mut change: [&str; 8] = ["none"; 8];
static mut changing: [bool; 8] = [false; 8];

static mut smashCD: [f32; 8] = [0.0; 8];
static mut busterCD: [f32; 8] = [0.0; 8];
static mut jumpCD: [f32; 8] = [0.0; 8];
static mut shieldCD: [f32; 8] = [0.0; 8];
static mut speedCD: [f32; 8] = [0.0; 8];

static mut totalTimer: [f32; 8] = [0.0; 8];
static mut timer: [f32; 8] = [0.0; 8];

static mut r: [f32; 8] = [0.0; 8];
static mut g: [f32; 8] = [0.0; 8];
static mut b: [f32; 8] = [0.0; 8];
static mut effect: [i32; 8] = [0; 8];

#[fighter_frame( agent = FIGHTER_KIND_DIDDY )]
fn diddy_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
let module_accessor = smash::app::sv_system::battle_object_module_accessor(fighter.lua_state_agent);
let entry_id = WorkModule::get_int(module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_ENTRY_ID) as usize;
let kinetic_motion = std::mem::transmute::<u64, &mut smash::app::FighterKineticEnergyMotion>(KineticModule::get_energy(module_accessor, *FIGHTER_KINETIC_ENERGY_ID_MOTION));
if(MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_shoot") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_blow") || MotionModule::motion_kind(module_accessor) == smash::hash40("attack_12") || MotionModule::motion_kind(module_accessor) == smash::hash40("attack_13") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_danger") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_charge")) {
	fighter.change_status(FIGHTER_STATUS_KIND_WALK.into(), false.into());
}
if(MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n_shoot") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n_blow") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_danger") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_charge")) {
	fighter.change_status(FIGHTER_STATUS_KIND_FALL.into(), false.into());
}
if(MotionModule::frame(module_accessor) == 5.0) {
	if(MotionModule::motion_kind(module_accessor) == smash::hash40("special_n_start") || MotionModule::motion_kind(module_accessor) == smash::hash40("special_air_n_start")) {
        if(ControlModule::get_stick_y(module_accessor) > 0.809 && smashCD[entry_id] <= 0.0) {
            if(state[entry_id] != "smash") {
                change[entry_id] = "smash";
				changing[entry_id] = true;
                totalTimer[entry_id] = 480.0; 
                r[entry_id] = 5.0;
                g[entry_id] = 0.0;
                b[entry_id] = 0.0;
            }
        }
		if(ControlModule::get_stick_y(module_accessor) <= 0.809 && ControlModule::get_stick_y(module_accessor) >= -0.309 && ControlModule::get_stick_x(module_accessor) >= 0.0 && busterCD[entry_id] <= 0.0) {
            if(state[entry_id] != "buster") {
                change[entry_id] = "buster";
				changing[entry_id] = true;
                totalTimer[entry_id] = 600.0; 
                r[entry_id] = 3.0;
				g[entry_id] = 0.0;
                b[entry_id] = 3.0;
            }
        }
		if(ControlModule::get_stick_y(module_accessor) <= 0.809 && ControlModule::get_stick_y(module_accessor) >= -0.309 && ControlModule::get_stick_x(module_accessor) < 0.0 && jumpCD[entry_id] <= 0.0) { 
			if(state[entry_id] != "jump") {
                change[entry_id] = "jump";
				changing[entry_id] = true;
                totalTimer[entry_id] = 360.0; 
                r[entry_id] = 0.0;
                g[entry_id] = 5.0;
				b[entry_id] = 0.0;
            }
        }
        if(ControlModule::get_stick_y(module_accessor) < -0.309 && ControlModule::get_stick_x(module_accessor) >= 0.0 && shieldCD[entry_id] <= 0.0) {
            if(state[entry_id] != "shield") {
                change[entry_id] = "shield";
                changing[entry_id] = true;
                totalTimer[entry_id] = 360.0; 
                r[entry_id] = 5.0;
				g[entry_id] = 5.0;
                b[entry_id] = 0.0;
            }
        }
        if(ControlModule::get_stick_y(module_accessor) < -0.309 && ControlModule::get_stick_x(module_accessor) < 0.0 && speedCD[entry_id] <= 0.0) {
            if(state[entry_id] != "speed") {
                change[entry_id] = "speed";
				changing[entry_id] = true;
                totalTimer[entry_id] = 480.0; 
                r[entry_id] = 0.0;
                g[entry_id] = 4.0;
				b[entry_id] = 4.0;
            }
        }
		if(ControlModule::get_stick_y(module_accessor) == 0.0 && ControlModule::get_stick_x(module_accessor) == 0.0) {
			if(jumpCD[entry_id] <= 0.0 && state[entry_id] != "jump") {
                change[entry_id] = "jump";
				changing[entry_id] = true;
                totalTimer[entry_id] = 360.0; 
                r[entry_id] = 0.0;
                g[entry_id] = 5.0;
				b[entry_id] = 0.0;
			} else if(speedCD[entry_id] <= 0.0 && state[entry_id] != "speed") {
                change[entry_id] = "speed";
				changing[entry_id] = true;
                totalTimer[entry_id] = 480.0; 
                r[entry_id] = 0.0;
                g[entry_id] = 4.0;
				b[entry_id] = 4.0;
			} else if(shieldCD[entry_id] <= 0.0 && state[entry_id] != "shield") {
                change[entry_id] = "shield";
				changing[entry_id] = true;
                totalTimer[entry_id] = 360.0; 
                r[entry_id] = 5.0;
				g[entry_id] = 5.0;
                b[entry_id] = 0.0;
			} else if(busterCD[entry_id] <= 0.0 && state[entry_id] != "buster") {
                change[entry_id] = "buster";
				changing[entry_id] = true;
                totalTimer[entry_id] = 600.0; 
                r[entry_id] = 3.0;
				g[entry_id] = 0.0;
                b[entry_id] = 3.0;
			} else if(smashCD[entry_id] <= 0.0 && state[entry_id] != "smash") {
                change[entry_id] = "smash";
				changing[entry_id] = true;
                totalTimer[entry_id] = 480.0; 
                r[entry_id] = 5.0;
                g[entry_id] = 0.0;
                b[entry_id] = 0.0;
			} 
		}
    }
}
if(state[entry_id] == "smash") { //smash
	r[entry_id] = 5.0;
	g[entry_id] = 0.0;
	b[entry_id] = 0.0;
	AttackModule::set_power_up(module_accessor, 0.3);
	AttackModule::set_reaction_mul(module_accessor, 1.25);
	DamageModule::set_damage_mul(module_accessor, 1.0);
	DamageModule::set_reaction_mul(module_accessor, 1.2);
	ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
}
if(state[entry_id] == "buster") { //buster
	r[entry_id] = 3.0;
	g[entry_id] = 0.0;
	b[entry_id] = 3.0;
	AttackModule::set_power_up(module_accessor, 1.4);
	AttackModule::set_reaction_mul(module_accessor, 0.65);
	DamageModule::set_damage_mul(module_accessor, 1.3);
	DamageModule::set_reaction_mul(module_accessor, 1.0);
	ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
	ShieldModule::set_hit_stop_mul(module_accessor, 1.0);
}
if(state[entry_id] == "jump") { //jump
	if(MotionModule::motion_kind(module_accessor) != smash::hash40("jump_f_mini") && MotionModule::motion_kind(module_accessor) != smash::hash40("jump_b_mini")) {
		r[entry_id] = 0.0;
		g[entry_id] = 5.0;
		b[entry_id] = 0.0;
		if(StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_JUMP_AERIAL || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI) {
            if(MotionModule::frame(module_accessor) < 1.0) {
				KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: 1.7, z: 0.0} as *const Vector3f);
			}
		}
        if(StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_JUMP) {
			if(MotionModule::frame(module_accessor) < 5.0) {
				KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: 1.7, z: 0.0} as *const Vector3f);
			}
		}
		if(StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_FALL) {
			KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: -0.12, z: 0.0} as *const Vector3f);
		}
		AttackModule::set_power_up(module_accessor, 1.0);
        AttackModule::set_reaction_mul(module_accessor, 1.0);
        DamageModule::set_damage_mul(module_accessor, 1.3);
        DamageModule::set_reaction_mul(module_accessor, 1.0);
        ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
        ShieldModule::set_hit_stop_mul(module_accessor, 1.0);
	}
}
if(state[entry_id] == "shield") { //shield
	if(MotionModule::motion_kind(module_accessor) != smash::hash40("jump_f_mini") && MotionModule::motion_kind(module_accessor) != smash::hash40("jump_b_mini")) {
		r[entry_id] = 5.0;
		g[entry_id] = 5.0;
		b[entry_id] = 0.0;
		if(StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_JUMP_AERIAL || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI) {
			if(MotionModule::frame(module_accessor) < 1.0) {
				KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: -0.16, z: 0.0} as *const Vector3f);
			}
		}
		if(StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_JUMP) {
			if(MotionModule::frame(module_accessor) < 5.0) {
				KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: -0.84, z: 0.0} as *const Vector3f);
			}
		}
		if(StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_AIR && StatusModule::status_kind(module_accessor) != *FIGHTER_STATUS_KIND_SPECIAL_HI) {
			let lua_state = fighter.lua_state_agent;
			acmd!(lua_state, {
				sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.6)
			});
			smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 0.6);
		}
		if(StatusModule::situation_kind(module_accessor) == *SITUATION_KIND_GROUND) {
			smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 0.6);
			let lua_state = fighter.lua_state_agent;
			acmd!(lua_state, {
				sv_kinetic_energy::mul_x_speed_max(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.6)
				sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.6)
			});
		}
		AttackModule::set_power_up(module_accessor, 0.5);
		AttackModule::set_reaction_mul(module_accessor, 0.8);
		DamageModule::set_damage_mul(module_accessor, 0.5);
		DamageModule::set_reaction_mul(module_accessor, 0.6);
		ShieldModule::set_attack_mul(module_accessor, 0.5, *FIGHTER_SHIELD_KIND_GUARD);
		ShieldModule::set_hit_stop_mul(module_accessor, 0.8);
	}
}
if(state[entry_id] == "speed") { //speed
	if(MotionModule::motion_kind(module_accessor) != smash::hash40("jump_f_mini") && MotionModule::motion_kind(module_accessor) != smash::hash40("jump_b_mini")) {
		r[entry_id] = 0.0;
		g[entry_id] = 4.0;
		b[entry_id] = 4.0;
        if(StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_JUMP_AERIAL || StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_SPECIAL_HI) {
            if(MotionModule::frame(module_accessor) < 1.0) {
				KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: -0.2, z: 0.0} as *const Vector3f);
			}
		}
		if(StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_JUMP) {
			if(MotionModule::frame(module_accessor) < 5.0) {
				KineticModule::add_speed(module_accessor, &Vector3f{x: 0.0, y: -1.0, z: 0.0} as *const Vector3f);
			}
		}
		let lua_state = fighter.lua_state_agent;
        acmd!(lua_state, {
			sv_kinetic_energy::set_accel_y_mul(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 0.25)
			sv_kinetic_energy::set_accel_x_mul(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.25)
			sv_kinetic_energy::set_limit_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 2.0)
			sv_kinetic_energy::set_stable_speed(FIGHTER_KINETIC_ENERGY_ID_CONTROL, 1.5)
		});
		smash::app::lua_bind::FighterKineticEnergyMotion::set_speed_mul(kinetic_motion, 2.0);
		AttackModule::set_power_up(module_accessor, 0.7);
		AttackModule::set_reaction_mul(module_accessor, 1.0);
		DamageModule::set_damage_mul(module_accessor, 1.0);
		DamageModule::set_reaction_mul(module_accessor, 1.0);
		ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
		ShieldModule::set_hit_stop_mul(module_accessor, 1.0);
	}
}
if(state[entry_id] == "none") { //none
	r[entry_id] = 0.0;
	g[entry_id] = 0.0;
	b[entry_id] = 0.0;
	AttackModule::set_power_up(module_accessor, 1.0);
	AttackModule::set_reaction_mul(module_accessor, 1.0);
	DamageModule::set_damage_mul(module_accessor, 1.0);
	DamageModule::set_reaction_mul(module_accessor, 1.0);
	ShieldModule::set_attack_mul(module_accessor, 1.0, *FIGHTER_SHIELD_KIND_GUARD);
	ShieldModule::set_hit_stop_mul(module_accessor, 1.0);
}
if(StatusModule::status_kind(module_accessor) == *FIGHTER_STATUS_KIND_DEAD || sv_information::is_ready_go() == false) {
	smashCD[entry_id] = 0.0;
	busterCD[entry_id] = 0.0;
	speedCD[entry_id] = 0.0;
	shieldCD[entry_id] = 0.0;
	jumpCD[entry_id] = 0.0;
	totalTimer[entry_id] = 0.0;
	change[entry_id] = "none";
	changing[entry_id] = true;
}
if(timer[entry_id] <= 0.0 && state[entry_id] != "none") {
	change[entry_id] = "none";
	totalTimer[entry_id] = 0.0;
	r[entry_id] = 2.5;
	b[entry_id] = 2.5;
	g[entry_id] = 2.5;
	changing[entry_id] = true;
}
if(timer[entry_id] > 0.0) {
	timer[entry_id] = timer[entry_id] - 1.0;
}
if(smashCD[entry_id] > 0.0) {
	smashCD[entry_id] = smashCD[entry_id] -1.0;
}
if(busterCD[entry_id] > 0.0) {
	busterCD[entry_id] = busterCD[entry_id] -1.0;
}
if(jumpCD[entry_id] > 0.0) {
	jumpCD[entry_id] = jumpCD[entry_id] -1.0;
}
if(shieldCD[entry_id] > 0.0) {
	shieldCD[entry_id] = shieldCD[entry_id] -1.0;
}
if(speedCD[entry_id] > 0.0) {
	speedCD[entry_id] = speedCD[entry_id] -1.0;
}
if(change[entry_id] == state[entry_id]) {
	change[entry_id] = "none";
	changing[entry_id] = false;
}
if(state[entry_id] != "none" && timer[entry_id] <= 0.0 || changing[entry_id] == true) {
	if(state[entry_id] == "smash") {
		smashCD[entry_id] = 960.0;
	}
	if(state[entry_id] == "buster") {
		busterCD[entry_id] = 840.0;
	}
	if(state[entry_id] == "jump") {
		jumpCD[entry_id] = 1080.0; 
	}
	if(state[entry_id] == "shield") {
		shieldCD[entry_id] = 1080.0;
	}
	if(state[entry_id] == "speed") {
		speedCD[entry_id] = 960.0;
	}
	timer[entry_id] = totalTimer[entry_id]; 
	state[entry_id] = change[entry_id]; 
	changing[entry_id] = false; 
}
effect[entry_id] += 1;
if(effect[entry_id] == 10) {
	macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
	macros::LAST_EFFECT_SET_COLOR(fighter, r[entry_id], g[entry_id], b[entry_id]);
}
if(effect[entry_id] >= 20) {
	macros::EFFECT_OFF_KIND(fighter, Hash40::new("sys_aura_light"), false, false);
	macros::EFFECT_FOLLOW_ALPHA(fighter, Hash40::new("sys_aura_light"), Hash40::new("waist"), 0, 0, 0, 0, 0, 0, 4.0, true, 1.0);
	macros::LAST_EFFECT_SET_COLOR(fighter, r[entry_id], g[entry_id], b[entry_id]);
	effect[entry_id] = 0;
}
    }
}

#[acmd_script( agent = "diddy", script = "game_specialnstart", category = ACMD_GAME, low_priority )]
unsafe fn diddy_specialnstart(fighter: &mut L2CAgentBase) {
}

#[acmd_script( agent = "diddy", script = "game_specialairnstart", category = ACMD_GAME, low_priority )]
unsafe fn diddy_specialairnstart(fighter: &mut L2CAgentBase) {
}

pub fn install() {
    smashline::install_agent_frames!(
        diddy_frame,
    );
	smashline::install_acmd_scripts!(
        diddy_specialnstart,
		diddy_specialairnstart,
    );
}