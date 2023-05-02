use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*
        
    },
    smash_script::*,
    smashline::*,
    smash::app::utility

};

static mut S_CANCEL: i32 = 0;
static mut S_SPEED: f32 = 0.0;
static mut ALLOW_CANCEL: bool = false;



#[fighter_frame( agent = FIGHTER_KIND_IKE )]
fn ike_frame(fighter: &mut L2CFighterCommon) {
    unsafe {
        let status = StatusModule::status_kind(fighter.module_accessor);

        if(S_CANCEL > 0){
            let speed_vector = smash::phx::Vector3f{x: (S_SPEED*PostureModule::lr(fighter.module_accessor))*1.5, y: 0.0, z: 0.0};
            KineticModule::add_speed(fighter.module_accessor, &speed_vector);
            S_CANCEL-=1;

            ALLOW_CANCEL = false;

        }
        
        if status == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_HOLD {
            ALLOW_CANCEL = true;
        }

        if (status == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_DASH ||
        status == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_ATTACK ||
        status == *FIGHTER_IKE_STATUS_KIND_SPECIAL_S_END) && ALLOW_CANCEL = true {
            if ControlModule::check_button_on(fighter.module_accessor, *CONTROL_PAD_BUTTON_JUMP) {

                if WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT) < WorkModule::get_int(fighter.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_JUMP_COUNT_MAX) && StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_AIR {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_AERIAL, true);
                    S_CANCEL = 3;
                    S_SPEED = (KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
                }; 
            
                if StatusModule::situation_kind(fighter.module_accessor) == *SITUATION_KIND_GROUND {
                    StatusModule::change_status_request_from_script(fighter.module_accessor, *FIGHTER_STATUS_KIND_JUMP_SQUAT, true);
                    S_CANCEL = 5;
                    S_SPEED = (KineticModule::get_sum_speed_x(fighter.module_accessor, *KINETIC_ENERGY_RESERVE_ATTRIBUTE_MAIN));
                };

                ControlModule::clear_command(fighter.module_accessor, true);


            }
        }
    }  
}


pub fn install() {
    smashline::install_acmd_scripts!(

    );

 
    smashline::install_agent_frames!(
        ike_frame
    );
}
