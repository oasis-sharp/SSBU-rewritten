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

#[fighter_frame( agent = FIGHTER_KIND_GEKKOUGA )]
fn gren_frame(fighter: &mut L2CFighterCommon) {
    unsafe{
        pub static mut DOLL_POS: [Vector3f; 8] = [Vector3f{x: 0.0, y: 0.0, z: 0.0}, 8];
        let doll_id = WorkModule::get_int(fighter.module_accessor, *FIGHTER_GEKKOUGA_INSTANCE_WORK_ID_INT_SPECIAL_LW_DOLL_ID);
        let doll_boma = sv_battle_object::module_accessor(doll_id);
        DOLL_POS[doll_id] = Vector3f {x: PostureModule::pos_x(doll_boma), y: PostureModule::pos_y(doll_boma), z: PostureModule::pos_z(doll_boma)}; // get current pos
    }
}



pub fn install() {
    smashline::install_acmd_scripts!(

    );

 
    smashline::install_agent_frames!(

    );

}
