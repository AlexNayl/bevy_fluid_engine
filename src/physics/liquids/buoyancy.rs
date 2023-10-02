use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

pub struct BuoyancyPlugin;

impl Plugin for BuoyancyPlugin{
    fn build(&self, app: &mut App) {
        app.add_systems(Update, buoyancy_system);
    }
}


fn buoyancy_system(
    rapier_context: Res<RapierContext>,
    config : ResMut<RapierConfiguration>,
    mut ridgidbody_query: Query<(Entity,&RigidBody,&Collider),Without<super::Liquid>>,
    liquid_query: Query<(&Collider, &super::Liquid)>
){
    ridgidbody_query.par_iter_mut().for_each_mut(|
            (entity, rigid_body, collider)
        |{
            for (collider1, collider2, intersecting) in rapier_context.intersections_with(entity) {
                if intersecting {
                    for (liquid_collider, liquid) in liquid_query.get(collider2).iter(){

                    }
                }
            }
        });
}