use super::{
    super::{Animation, vek::*},
    BipedLargeSkeleton, SkeletonAttr,
};
use common::{
    comp::item::tool::{AbilitySpec, ToolKind},
    states::utils::StageSection,
};
use core::f32::consts::PI;

pub struct ShootAnimation;

type ShootAnimationDependency<'a> = (
    Option<ToolKind>,
    (Option<ToolKind>, Option<&'a AbilitySpec>),
    Vec3<f32>,
    Vec3<f32>,
    Vec3<f32>,
    f32,
    Option<StageSection>,
    f32,
    Option<&'a str>,
);
impl Animation for ShootAnimation {
    type Dependency<'a> = ShootAnimationDependency<'a>;
    type Skeleton = BipedLargeSkeleton;

    #[cfg(feature = "use-dyn-lib")]
    const UPDATE_FN: &'static [u8] = b"biped_large_shoot\0";

    #[cfg_attr(feature = "be-dyn-lib", unsafe(export_name = "biped_large_shoot"))]
    fn update_skeleton_inner(
        skeleton: &Self::Skeleton,
        (
            active_tool_kind,
            _second_tool,
            velocity,
            _orientation,
            _last_ori,
            _global_time,
            stage_section,
            acc_vel,
            ability_id,
        ): Self::Dependency<'_>,
        anim_time: f32,
        rate: &mut f32,
        s_a: &SkeletonAttr,
    ) -> Self::Skeleton {
        *rate = 1.0;
        let speed = Vec2::<f32>::from(velocity).magnitude();

        let mut next = (*skeleton).clone();

        let lab: f32 = 0.65 * s_a.tempo;
        let speednorm = (speed / 12.0).powf(0.4);
        let foothoril = (acc_vel * lab + PI * 1.45).sin() * speednorm;
        let foothorir = (acc_vel * lab + PI * (0.45)).sin() * speednorm;
        let footrotl = ((1.0 / (0.5 + (0.5) * ((acc_vel * lab + PI * 1.4).sin()).powi(2))).sqrt())
            * ((acc_vel * lab + PI * 1.4).sin())
            * speednorm;

        let footrotr = ((1.0 / (0.5 + (0.5) * ((acc_vel * lab + PI * 0.4).sin()).powi(2))).sqrt())
            * ((acc_vel * lab + PI * 0.4).sin())
            * speednorm;

        next.shoulder_l.position = Vec3::new(
            -s_a.shoulder.0,
            s_a.shoulder.1,
            s_a.shoulder.2 - foothorir * 1.0,
        );
        next.shoulder_l.orientation =
            Quaternion::rotation_x(0.8 + 1.2 * speednorm + (footrotr * -0.2) * speednorm);

        next.shoulder_r.position = Vec3::new(
            s_a.shoulder.0,
            s_a.shoulder.1,
            s_a.shoulder.2 - foothoril * 1.0,
        );
        next.shoulder_r.orientation =
            Quaternion::rotation_x(0.8 + 1.2 * speednorm + (footrotl * -0.2) * speednorm);
        next.jaw.position = Vec3::new(0.0, s_a.jaw.0, s_a.jaw.1);
        next.jaw.orientation = Quaternion::rotation_x(0.0);

        next.main.position = Vec3::new(0.0, 0.0, 0.0);
        next.main.orientation = Quaternion::rotation_x(0.0);

        next.hand_l.position = Vec3::new(0.0, 0.0, s_a.grip.0);
        next.hand_r.position = Vec3::new(0.0, 0.0, s_a.grip.0);

        next.hand_l.orientation = Quaternion::rotation_x(0.0);
        next.hand_r.orientation = Quaternion::rotation_x(0.0);

        match active_tool_kind {
            Some(ToolKind::Sword) => match ability_id {
                Some(
                    "common.abilities.custom.dullahan.knife_rain"
                    | "common.abilities.custom.dullahan.fierce_darts",
                ) => {
                    let (move1base, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time.powf(0.25), 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time, 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time.powi(4)),
                        _ => (0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1 = move1base * pullback;
                    let move2 = move2base * pullback;
                    next.main.position = Vec3::new(-10.0, -8.0, 12.0);
                    next.main.orientation =
                        Quaternion::rotation_y(2.5) * Quaternion::rotation_z(PI / 2.0);
                    next.hand_l.position = Vec3::new(-s_a.hand.0, s_a.hand.1 + 4.0, s_a.hand.2);
                    next.hand_r.position = Vec3::new(s_a.hand.0, s_a.hand.1 + 4.0, s_a.hand.2);
                    next.hand_l.orientation = Quaternion::rotation_x(move1 * 1.5)
                        * Quaternion::rotation_y(move1 * -1.0 + move2 * 1.5);
                    next.hand_r.orientation = Quaternion::rotation_x(move1 * 1.5)
                        * Quaternion::rotation_y(move1 * 1.0 + move2 * -1.5);
                    next.upper_torso.orientation =
                        Quaternion::rotation_y(move1 * -0.1 + move2 * 0.1)
                            * Quaternion::rotation_z(move1 * -0.1 + move2 * 0.1);
                    next.foot_l.orientation = Quaternion::rotation_y(move1 * 0.3 + move2 * -0.3);
                    next.foot_r.orientation = Quaternion::rotation_y(move1 * 0.3 + move2 * -0.3);
                },
                Some("common.abilities.adlet.elder.air_blade") => {
                    let (move1base, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time, 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time.powf(0.25), 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time),
                        _ => (0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1abs = move1base * pullback;
                    let move2abs = move2base * pullback;
                    next.main.position = Vec3::new(-10.0, -8.0, 12.0);
                    next.main.orientation =
                        Quaternion::rotation_y(2.5) * Quaternion::rotation_z(PI / 2.0);

                    next.hand_l.position =
                        Vec3::new(-s_a.hand.0, s_a.hand.1 + 1.0, s_a.hand.2 + 5.0);
                    next.hand_r.position =
                        Vec3::new(s_a.hand.0, s_a.hand.1 + 1.0, s_a.hand.2 + 5.0);

                    next.hand_r.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7)
                            * Quaternion::rotation_y(0.0 + move1abs * -0.7);
                    next.hand_l.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7);
                    next.hand_r.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7);

                    next.shoulder_l.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7);
                    next.shoulder_r.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7);
                    next.head.orientation =
                        Quaternion::rotation_x(move1abs * 0.4 + move2abs * -0.2);
                },
                Some("common.abilities.adlet.elder.trap") => {
                    let (move1base, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time, 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time.powf(0.25), 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time),
                        _ => (0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1abs = move1base * pullback;
                    let move2abs = move2base * pullback;
                    //   next.main.position = Vec3::new(-10.0, -8.0, 12.0);
                    //                    next.main.orientation = Quaternion::rotation_y(2.5) *
                    // Quaternion::rotation_z(PI / 2.0);

                    next.hand_l.position =
                        Vec3::new(-s_a.hand.0, s_a.hand.1 + 1.0, s_a.hand.2 + 5.0);
                    next.hand_r.position =
                        Vec3::new(s_a.hand.0, s_a.hand.1 + 1.0, s_a.hand.2 + 5.0);

                    next.hand_r.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7)
                            * Quaternion::rotation_y(0.0 + move1abs * -0.7);
                    next.hand_l.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7);
                    next.hand_r.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7);

                    next.shoulder_l.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7);
                    next.shoulder_r.orientation =
                        Quaternion::rotation_x(move1abs * 4.0 + move2abs * -0.7);
                    next.head.orientation =
                        Quaternion::rotation_x(move1abs * 0.4 + move2abs * -0.2);
                },
                _ => {},
            },
            Some(ToolKind::Hammer) => match ability_id {
                Some("common.abilities.custom.cyclops.optic_blast") => {
                    let (move1base, _move1shake, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => {
                            (anim_time, (anim_time * 10.0 + PI).sin(), 0.0, 0.0)
                        },
                        Some(StageSection::Action) => (1.0, 1.0, anim_time.powf(0.25), 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, 1.0, anim_time),
                        _ => (0.0, 0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1 = move1base * pullback;
                    let move2 = move2base * pullback;
                    next.head.orientation = Quaternion::rotation_x(move1 * 0.25 + move2 * -0.25)
                        * Quaternion::rotation_z(move1 * 0.25);
                    next.torso.orientation = Quaternion::rotation_x(move1 * -0.25 + move2 * 0.25);
                    next.upper_torso.orientation =
                        Quaternion::rotation_x(move1 * -0.1 + move2 * 0.1)
                            * Quaternion::rotation_z(move1 * -0.1 + move2 * 0.1);
                    next.foot_l.orientation = Quaternion::rotation_x(move1 * 0.3 + move2 * -0.3);
                    next.foot_r.orientation = Quaternion::rotation_x(move1 * 0.3 + move2 * -0.3);
                    next.main.position = Vec3::new(0.0, -10.0, 3.0);
                    next.main.orientation = Quaternion::rotation_x(PI / -2.0);
                    next.weapon_l.position = Vec3::new(
                        -s_a.hand.0 - 3.0 * move1,
                        s_a.hand.1 + 4.0 + 8.0 * move1,
                        -15.0 + 2.0 * move1,
                    );
                    next.weapon_l.orientation = Quaternion::rotation_x(move1 * 0.6);
                    next.hand_r.position = Vec3::new(
                        s_a.hand.0 + 6.0 * move1,
                        s_a.hand.1 + 4.0,
                        s_a.hand.2 + 6.0 * move1,
                    );
                    next.hand_r.orientation =
                        Quaternion::rotation_x(move1 * 1.0) * Quaternion::rotation_y(move1 * -1.4);

                    next.shoulder_l.orientation =
                        Quaternion::rotation_x(move1 * 0.6) * Quaternion::rotation_y(move1 * 0.5);
                    next.shoulder_r.orientation =
                        Quaternion::rotation_x(move1 * 1.4) * Quaternion::rotation_y(move1 * -0.5);
                },
                Some(
                    "common.abilities.custom.dwarves.forgemaster.lava_mortar"
                    | "common.abilities.custom.dwarves.forgemaster.mines",
                ) => {
                    let (move1base, move2shake, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => {
                            ((anim_time.powf(0.25)).min(1.0), 0.0, 0.0, 0.0)
                        },
                        Some(StageSection::Action) => (
                            1.0,
                            (anim_time * 15.0 + PI).sin(),
                            (anim_time.powf(0.1)).min(1.0),
                            0.0,
                        ),
                        Some(StageSection::Recover) => (1.0, 1.0, 1.0, anim_time),
                        _ => (0.0, 0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1 = move1base * pullback;
                    let _move2 = move2base * pullback;
                    next.control_l.position = Vec3::new(-1.0, 3.0, 6.0);
                    next.control_r.position =
                        Vec3::new(-1.0 + move1 * 5.0, 2.0 + move1 * 1.0, 2.0 + move1 * 8.0);

                    next.control.position = Vec3::new(
                        -3.0 + move1 * -5.0,
                        -2.0 + s_a.grip.0 / 1.2 + move1 * 3.0 + move2shake * 1.0,
                        8.0 + -s_a.grip.0 / 2.0 + move1 * -2.0,
                    );
                    next.head.orientation =
                        Quaternion::rotation_x(move1 * -0.2) * Quaternion::rotation_y(move1 * 0.2);
                    next.jaw.orientation = Quaternion::rotation_x(0.0);

                    next.control_l.orientation =
                        Quaternion::rotation_x(PI / 2.0) * Quaternion::rotation_y(-0.5);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 2.5 + move1 * 0.4)
                        * Quaternion::rotation_y(1.0)
                        * Quaternion::rotation_z(move1 * 1.2 + move2shake * 0.5);

                    next.control.orientation = Quaternion::rotation_x(-0.2 + move1 * -0.1)
                        * Quaternion::rotation_y(-0.1 + move1 * 0.3);
                    next.shoulder_l.position = Vec3::new(
                        -s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothorir * 1.0,
                    );
                    next.shoulder_l.orientation = Quaternion::rotation_x(
                        move1 * 0.2 + 0.3 + 0.8 * speednorm + (footrotr * -0.2),
                    );
                    next.shoulder_r.position = Vec3::new(
                        s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothoril * 1.0,
                    );
                    next.shoulder_r.orientation = Quaternion::rotation_x(
                        move1 * 0.2 + 1.1 + 0.6 * speednorm + (footrotl * -0.2),
                    );
                },
                _ => {},
            },
            Some(ToolKind::Sceptre) => {
                let (move1base, move1shake, move2base, move3) = match stage_section {
                    Some(StageSection::Buildup) => {
                        (anim_time, (anim_time * 10.0 + PI).sin(), 0.0, 0.0)
                    },
                    Some(StageSection::Action) => (1.0, 1.0, anim_time.powf(0.25), 0.0),
                    Some(StageSection::Recover) => (1.0, 1.0, 1.0, anim_time),
                    _ => (0.0, 0.0, 0.0, 0.0),
                };
                let pullback = 1.0 - move3;
                let move1 = move1base * pullback;
                let move2 = move2base * pullback;
                next.control_l.position = Vec3::new(-1.0, 3.0, 12.0);
                next.control_r.position = Vec3::new(1.0, 2.0, 2.0);

                next.control.position = Vec3::new(
                    -3.0,
                    3.0 + s_a.grip.0 / 1.2 + move1 * 4.0 + move2 + move1shake * 2.0 + move2 * -2.0,
                    -11.0 + -s_a.grip.0 / 2.0 + move1 * 3.0,
                );
                next.head.orientation = Quaternion::rotation_x(move1 * -0.15)
                    * Quaternion::rotation_y(move1 * 0.25)
                    * Quaternion::rotation_z(move1 * 0.25);
                next.jaw.orientation = Quaternion::rotation_x(move1 * -0.5);

                next.control_l.orientation = Quaternion::rotation_x(PI / 2.0 + move1 * 0.5)
                    * Quaternion::rotation_y(move1 * -0.4);
                next.control_r.orientation = Quaternion::rotation_x(PI / 2.5 + move1 * 0.5)
                    * Quaternion::rotation_y(0.5)
                    * Quaternion::rotation_z(0.0);

                next.control.orientation =
                    Quaternion::rotation_x(-0.2 + move1 * -0.2 + move1shake * 0.1)
                        * Quaternion::rotation_y(-0.1 + move1 * 0.8 + move2 * -0.3);
                next.shoulder_l.position = Vec3::new(
                    -s_a.shoulder.0,
                    s_a.shoulder.1,
                    s_a.shoulder.2 - foothorir * 1.0,
                );
                next.shoulder_l.orientation = Quaternion::rotation_x(
                    move1 * 0.8 + 0.8 * speednorm + (footrotr * -0.2) * speednorm,
                );

                next.shoulder_r.position = Vec3::new(
                    s_a.shoulder.0,
                    s_a.shoulder.1,
                    s_a.shoulder.2 - foothoril * 1.0,
                );
                next.shoulder_r.orientation =
                    Quaternion::rotation_x(move1 * 0.8 + 0.6 * speednorm + (footrotl * -0.2));
            },
            Some(ToolKind::Staff) => match ability_id {
                Some("common.abilities.custom.mindflayer.necroticsphere_multiblast") => {
                    let (move1base, move1shake, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => {
                            (anim_time, (anim_time * 10.0 + PI).sin(), 0.0, 0.0)
                        },
                        Some(StageSection::Action) => (1.0, 1.0, anim_time.powf(0.25), 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, 1.0, anim_time),
                        _ => (0.0, 0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1 = move1base * pullback;
                    let move2 = move2base * pullback;
                    let rotate = move1 * 2.0 * PI + move2 * 2.0 * PI;
                    let rise = move1 * 20.0 - move2 * 20.0;

                    next.control_l.position = Vec3::new(-1.0, 3.0, 12.0);
                    next.control_r.position = Vec3::new(1.0, 2.0, 2.0);

                    next.control.position = Vec3::new(
                        -3.0,
                        3.0 + s_a.grip.0 / 1.2
                            + move1 * 4.0
                            + move2
                            + move1shake * 2.0
                            + move2 * -2.0,
                        -11.0 + -s_a.grip.0 / 2.0 + move1 * 3.0,
                    );
                    next.head.orientation = Quaternion::rotation_x(move1 * -0.15)
                        * Quaternion::rotation_y(move1 * 0.25)
                        * Quaternion::rotation_z(move1 * 0.25);
                    next.jaw.orientation = Quaternion::rotation_x(move1 * -0.5);

                    next.upper_torso.orientation = Quaternion::rotation_z(rotate);
                    next.upper_torso.position = Vec3::new(0.0, 0.0, s_a.upper_torso.1 + rise);

                    next.control_l.orientation = Quaternion::rotation_x(PI / 2.0 + move1 * 0.5)
                        * Quaternion::rotation_y(move1 * -0.4);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 2.5 + move1 * 0.5)
                        * Quaternion::rotation_y(0.5)
                        * Quaternion::rotation_z(0.0);

                    next.control.orientation =
                        Quaternion::rotation_x(-0.2 + move1 * -0.2 + move1shake * 0.1)
                            * Quaternion::rotation_y(-0.1 + move1 * 0.8 + move2 * -0.3);
                    next.shoulder_l.position = Vec3::new(
                        -s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothorir * 1.0,
                    );
                    next.shoulder_l.orientation = Quaternion::rotation_x(
                        move1 * 0.8 + 0.8 * speednorm + (footrotr * -0.2) * speednorm,
                    );

                    next.shoulder_r.position = Vec3::new(
                        s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothoril * 1.0,
                    );
                    next.shoulder_r.orientation =
                        Quaternion::rotation_x(move1 * 0.8 + 0.6 * speednorm + (footrotl * -0.2));
                },
                _ => {
                    let (move1base, move1shake, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => {
                            (anim_time, (anim_time * 10.0 + PI).sin(), 0.0, 0.0)
                        },
                        Some(StageSection::Action) => (1.0, 1.0, anim_time.powf(0.25), 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, 1.0, anim_time),
                        _ => (0.0, 0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1 = move1base * pullback;
                    let move2 = move2base * pullback;
                    next.control_l.position = Vec3::new(-1.0, 3.0, 12.0);
                    next.control_r.position = Vec3::new(1.0, 2.0, 2.0);

                    next.control.position = Vec3::new(
                        -3.0,
                        3.0 + s_a.grip.0 / 1.2
                            + move1 * 4.0
                            + move2
                            + move1shake * 2.0
                            + move2 * -2.0,
                        -11.0 + -s_a.grip.0 / 2.0 + move1 * 3.0,
                    );
                    next.head.orientation = Quaternion::rotation_x(move1 * -0.15)
                        * Quaternion::rotation_y(move1 * 0.25)
                        * Quaternion::rotation_z(move1 * 0.25);
                    next.jaw.orientation = Quaternion::rotation_x(move1 * -0.5);

                    next.control_l.orientation = Quaternion::rotation_x(PI / 2.0 + move1 * 0.5)
                        * Quaternion::rotation_y(move1 * -0.4);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 2.5 + move1 * 0.5)
                        * Quaternion::rotation_y(0.5)
                        * Quaternion::rotation_z(0.0);

                    next.control.orientation =
                        Quaternion::rotation_x(-0.2 + move1 * -0.2 + move1shake * 0.1)
                            * Quaternion::rotation_y(-0.1 + move1 * 0.8 + move2 * -0.3);
                    next.shoulder_l.position = Vec3::new(
                        -s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothorir * 1.0,
                    );
                    next.shoulder_l.orientation = Quaternion::rotation_x(
                        move1 * 0.8 + 0.8 * speednorm + (footrotr * -0.2) * speednorm,
                    );

                    next.shoulder_r.position = Vec3::new(
                        s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothoril * 1.0,
                    );
                    next.shoulder_r.orientation =
                        Quaternion::rotation_x(move1 * 0.8 + 0.6 * speednorm + (footrotl * -0.2));
                },
            },
            Some(ToolKind::Bow) => match ability_id {
                Some("common.abilities.custom.terracotta_besieger.multishot") => {
                    let (move1base, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time.powf(0.25), 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time, 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time.powi(4)),
                        _ => (0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1 = move1base * pullback;
                    let move2 = move2base * pullback;
                    next.control_l.position = Vec3::new(-5.0, -2.0 + move2 * -7.0, -3.0);
                    next.control_r.position = Vec3::new(4.0, 4.0, 1.0);

                    next.control.position = Vec3::new(
                        -1.0 + move1 * 2.0,
                        6.0 + s_a.grip.0 / 1.2 + move1 * 7.0,
                        -5.0 + -s_a.grip.0 / 2.0 + move1 * s_a.height * 1.5,
                    );

                    next.control_l.orientation =
                        Quaternion::rotation_x(move1 * 0.2 + PI / 2.0 + move2 * 0.4)
                            * Quaternion::rotation_y(-0.2);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 2.2 + move1 * 0.4)
                        * Quaternion::rotation_y(0.4)
                        * Quaternion::rotation_z(0.0);

                    next.control.orientation = Quaternion::rotation_x(-0.2)
                        * Quaternion::rotation_y(2.0 + move1 * -0.4)
                        * Quaternion::rotation_z(0.1);
                    next.head.orientation = Quaternion::rotation_z(move1 * 0.25);
                    next.shoulder_l.position = Vec3::new(
                        -s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothorir * 1.0,
                    );
                    next.shoulder_l.orientation =
                        Quaternion::rotation_x(move1 * 1.2 + 1.2 * speednorm + (footrotr * -0.2));

                    next.shoulder_r.position = Vec3::new(
                        s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothoril * 1.0,
                    );
                    next.shoulder_r.orientation =
                        Quaternion::rotation_x(move1 * 0.8 + 1.2 * speednorm + (footrotl * -0.2));
                },
                _ => {
                    let (move1base, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time.powf(0.25), 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time, 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time.powi(4)),
                        _ => (0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1 = move1base * pullback;
                    let move2 = move2base * pullback;
                    next.control_l.position = Vec3::new(-1.0, -2.0 + move2 * -7.0, -3.0);
                    next.control_r.position = Vec3::new(0.0, 4.0, 1.0);

                    next.control.position = Vec3::new(
                        -1.0 + move1 * 2.0,
                        6.0 + s_a.grip.0 / 1.2 + move1 * 7.0,
                        -5.0 + -s_a.grip.0 / 2.0 + move1 * s_a.height * 3.4,
                    );

                    next.control_l.orientation =
                        Quaternion::rotation_x(move1 * 0.2 + PI / 2.0 + move2 * 0.4)
                            * Quaternion::rotation_y(-0.2);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 2.2 + move1 * 0.4)
                        * Quaternion::rotation_y(0.4)
                        * Quaternion::rotation_z(0.0);

                    next.control.orientation = Quaternion::rotation_x(-0.2)
                        * Quaternion::rotation_y(1.0 + move1 * -0.4)
                        * Quaternion::rotation_z(-0.1);
                    next.head.orientation = Quaternion::rotation_z(move1 * 0.25);
                    next.shoulder_l.position = Vec3::new(
                        -s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothorir * 1.0,
                    );
                    next.shoulder_l.orientation =
                        Quaternion::rotation_x(move1 * 1.2 + 1.2 * speednorm + (footrotr * -0.2));

                    next.shoulder_r.position = Vec3::new(
                        s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothoril * 1.0,
                    );
                    next.shoulder_r.orientation =
                        Quaternion::rotation_x(move1 * 0.8 + 1.2 * speednorm + (footrotl * -0.2));
                },
            },
            Some(ToolKind::Axe) => {
                let (move1base, move2, move3) = match stage_section {
                    Some(StageSection::Buildup) => ((anim_time.powf(0.25)), 0.0, 0.0),
                    Some(StageSection::Action) => (1.0, (anim_time), 0.0),
                    Some(StageSection::Recover) => (1.0, 1.0, anim_time),
                    _ => (0.0, 0.0, 0.0),
                };
                let pullback = 1.0 - move3;
                let move1 = move1base * pullback;
                let move2 = move2 * pullback;

                next.shoulder_r.orientation =
                    Quaternion::rotation_y(move1 * -0.5) * Quaternion::rotation_x(move1 * -0.5);
                next.head.orientation = Quaternion::rotation_x(0.0)
                    * Quaternion::rotation_y(move1 * 0.3)
                    * Quaternion::rotation_z(move1 * -0.2 + move2 * 0.5);

                next.main.position = Vec3::new(0.0, 0.0, 0.0);
                next.main.orientation = Quaternion::rotation_z(move1 * 5.0);

                next.hand_l.position = Vec3::new(s_a.grip.1, 0.0, s_a.grip.0);
                next.hand_r.position = Vec3::new(-s_a.grip.1, 0.0, s_a.grip.0);

                next.hand_l.orientation = Quaternion::rotation_x(0.0);
                next.hand_r.orientation = Quaternion::rotation_x(0.0);

                next.control_l.position = Vec3::new(-1.0, 2.0, 12.0);
                next.control_r.position = Vec3::new(1.0 + move1 * 40.0, 2.0, -2.0 + move1 * 10.0);

                next.control.position = Vec3::new(
                    4.0 + move1 * -25.0,
                    0.0 + s_a.grip.0 / 1.0 + move1 * -6.0,
                    -s_a.grip.0 / 0.8 + move1 * 10.0,
                );

                next.control_l.orientation =
                    Quaternion::rotation_x(PI / 2.0 + move1 * 0.3) * Quaternion::rotation_y(-0.0);
                next.control_r.orientation = Quaternion::rotation_x(PI / 2.0 + 0.2 + move1 * 1.0)
                    * Quaternion::rotation_y(0.0)
                    * Quaternion::rotation_z(0.0);
                next.control.orientation = Quaternion::rotation_x(-1.0 + move1 * 0.0)
                    * Quaternion::rotation_y(-1.8 + move1 * 2.0)
                    * Quaternion::rotation_z(0.0 + move1 * -0.0);
                next.upper_torso.orientation = Quaternion::rotation_y(move1 * 0.3);

                next.lower_torso.orientation = Quaternion::rotation_y(move1 * -0.3);
                next.torso.position = Vec3::new(move1, 0.0, 0.0);
            },
            Some(ToolKind::Natural) => match ability_id {
                Some("common.abilities.custom.minotaur.axethrow") => {
                    let (move1base, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time, 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time.powf(0.25), 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time),
                        _ => (0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1abs = move1base * pullback;
                    let move2abs = move2base * pullback;
                    next.second.scale = Vec3::one() * 1.0;
                    next.main.position = Vec3::new(-12.0, -4.0, -20.0);
                    next.second.position = Vec3::new(12.0, -4.0, -20.0);
                    next.main.orientation =
                        Quaternion::rotation_x(move1abs * -1.5 + move2abs * -3.5);

                    next.second.orientation =
                        Quaternion::rotation_x(move1abs * -1.5 + move2abs * -3.5);

                    if matches!(stage_section, Some(StageSection::Recover)) {
                        next.main.position += Vec3::new(0.0, 10000000.0, 0.0);
                        next.second.position += Vec3::new(0.0, 10000000.0, 0.0);
                    }

                    next.hand_l.position =
                        Vec3::new(-s_a.hand.0, s_a.hand.1 - 2.0, s_a.hand.2 + 0.0);
                    next.hand_r.position =
                        Vec3::new(s_a.hand.0, s_a.hand.1 - 2.0, s_a.hand.2 + 0.0);
                    next.control.orientation =
                        Quaternion::rotation_x(move1abs * 3.0 + move2abs * -3.0);

                    next.shoulder_l.orientation =
                        Quaternion::rotation_x(move1abs * 3.0 + move2abs * -3.0);
                    next.shoulder_r.orientation =
                        Quaternion::rotation_x(move1abs * 3.0 + move2abs * -3.0);
                    next.head.orientation =
                        Quaternion::rotation_x(move1abs * 0.4 + move2abs * -0.2);
                },
                Some("common.abilities.custom.wendigomagic.frostbomb") => {
                    let (move1base, _move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time.powf(0.25), 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time, 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time.powi(4)),
                        _ => (0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1 = move1base * pullback;
                    next.control_l.position =
                        Vec3::new(-9.0 + move1 * 6.0, 19.0 + move1 * 6.0, -13.0 + move1 * 10.5);
                    next.control_r.position =
                        Vec3::new(9.0 + move1 * -6.0, 19.0 + move1 * 6.0, -13.0 + move1 * 14.5);

                    next.control_l.orientation = Quaternion::rotation_x(PI / 3.0 + move1 * 0.5)
                        * Quaternion::rotation_y(-0.15)
                        * Quaternion::rotation_z(move1 * 0.5);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 3.0 + move1 * 0.5)
                        * Quaternion::rotation_y(0.15)
                        * Quaternion::rotation_z(move1 * -0.5);
                    next.head.orientation = Quaternion::rotation_x(move1 * -0.3);
                },
                Some("common.abilities.custom.yeti.snowball") => {
                    let (move1, move2, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time.powf(0.25), 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time, 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time.powi(4)),
                        _ => (0.0, 0.0, 0.0),
                    };
                    next.second.scale = Vec3::one() * 0.0;

                    next.head.orientation = Quaternion::rotation_x(move1 * 0.4);
                    next.jaw.position = Vec3::new(0.0, s_a.jaw.0, s_a.jaw.1);
                    next.jaw.orientation = Quaternion::rotation_x(move2 * -0.3);
                    next.control_l.position = Vec3::new(-0.5, 4.0, 1.0);
                    next.control_r.position = Vec3::new(-0.5, 4.0, 1.0);
                    next.control_l.orientation = Quaternion::rotation_x(PI / 2.0);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 2.0);
                    next.weapon_l.position = Vec3::new(-12.0, -1.0, -15.0);
                    next.weapon_r.position = Vec3::new(12.0, -1.0, -15.0);

                    next.weapon_l.orientation = Quaternion::rotation_x(-PI / 2.0 - 0.1);
                    next.weapon_r.orientation = Quaternion::rotation_x(-PI / 2.0 - 0.1);

                    let twist = move1 * 0.8 + move3 * -0.8;
                    next.upper_torso.position =
                        Vec3::new(0.0, s_a.upper_torso.0, s_a.upper_torso.1);
                    next.upper_torso.orientation =
                        Quaternion::rotation_x(move1 * 0.8 + move2 * -1.1)
                            * Quaternion::rotation_z(twist * -0.2 + move1 * -0.1 + move2 * 0.3);

                    next.lower_torso.orientation =
                        Quaternion::rotation_x(move1 * -0.8 + move2 * 1.1)
                            * Quaternion::rotation_z(twist);

                    next.arm_control_r.orientation = Quaternion::rotation_x(move1 * PI / 2.0)
                        * Quaternion::rotation_y(move1 * -PI / 2.0 + move2 * 2.5);
                    //* Quaternion::rotation_y(move1 * -PI/2.0)
                    //* Quaternion::rotation_z(move1 * -PI/2.0);
                    next.arm_control_r.position = Vec3::new(0.0, move1 * 10.0 + move2 * -10.0, 0.0);
                },
                Some("common.abilities.custom.terracotta_demolisher.throw") => {
                    let (move1, move2, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time.powf(0.25), 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time, 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time.powi(4)),
                        _ => (0.0, 0.0, 0.0),
                    };
                    next.head.orientation = Quaternion::rotation_x(move1 * 0.4);
                    next.control_l.position = Vec3::new(-0.5, 4.0, 1.0);
                    next.control_r.position = Vec3::new(-0.5, 4.0, 1.0);
                    next.control_l.orientation = Quaternion::rotation_x(PI / 1.5);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 1.5);
                    next.weapon_l.position = Vec3::new(-9.0, 5.0, 0.0);
                    next.weapon_r.position = Vec3::new(9.0, 5.0, 0.0);

                    next.weapon_l.orientation = Quaternion::rotation_x(-PI / 2.0 - 0.1);
                    next.weapon_r.orientation = Quaternion::rotation_x(-PI / 2.0 - 0.1);

                    let twist = move1 * 0.8 + move3 * -0.8;
                    next.upper_torso.position =
                        Vec3::new(0.0, s_a.upper_torso.0, s_a.upper_torso.1);
                    next.upper_torso.orientation =
                        Quaternion::rotation_x(move1 * 0.8 + move2 * -1.1)
                            * Quaternion::rotation_z(twist * -0.2 + move1 * -0.1 + move2 * 0.3);

                    next.lower_torso.orientation =
                        Quaternion::rotation_x(move1 * -0.8 + move2 * 1.1)
                            * Quaternion::rotation_z(twist);

                    next.arm_control_r.orientation = Quaternion::rotation_x(move1 * PI / 2.0)
                        * Quaternion::rotation_y(move1 * -PI / 3.0 + move2 * 1.5);
                    next.arm_control_r.position = Vec3::new(0.0, move1 * 1.0 + move2 * -1.0, 0.0);
                },
                Some("common.abilities.custom.terracotta_demolisher.drop") => {
                    let (move1base, move2base, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time.powf(0.25), 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time, 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time.powi(4)),
                        _ => (0.0, 0.0, 0.0),
                    };
                    let pullback = 1.0 - move3;
                    let move1 = move1base * pullback;
                    let move2 = move2base * pullback;
                    next.main.position = Vec3::new(-10.0, -8.0, 12.0);
                    next.main.orientation =
                        Quaternion::rotation_y(2.5) * Quaternion::rotation_z(PI / 2.0);
                    next.hand_l.position = Vec3::new(-s_a.hand.0, s_a.hand.1 + 4.0, s_a.hand.2);
                    next.hand_r.position = Vec3::new(s_a.hand.0, s_a.hand.1 + 4.0, s_a.hand.2);
                    next.hand_l.orientation = Quaternion::rotation_x(move1 * 1.5)
                        * Quaternion::rotation_y(move1 * -1.0 + move2 * 1.5);
                    next.hand_r.orientation = Quaternion::rotation_x(move1 * 1.5)
                        * Quaternion::rotation_y(move1 * 1.0 + move2 * -1.5);
                    next.upper_torso.orientation =
                        Quaternion::rotation_y(move1 * -0.1 + move2 * 0.1)
                            * Quaternion::rotation_z(move1 * -0.1 + move2 * 0.1);
                    next.foot_l.orientation = Quaternion::rotation_y(move1 * 0.3 + move2 * -0.3);
                    next.foot_r.orientation = Quaternion::rotation_y(move1 * 0.3 + move2 * -0.3);
                },
                Some("common.abilities.custom.harvester.explodingpumpkin") => {
                    let (move1, move2, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time.powf(0.25), 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time, 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time.powi(4)),
                        _ => (0.0, 0.0, 0.0),
                    };
                    next.control_l.position = Vec3::new(1.0, 2.0, 8.0);
                    next.control_r.position = Vec3::new(1.0, 1.0, -2.0);

                    next.control.position =
                        Vec3::new(-7.0, 0.0 + s_a.grip.0 / 1.0, -s_a.grip.0 / 0.8);

                    next.control_l.orientation =
                        Quaternion::rotation_x(PI / 2.0) * Quaternion::rotation_z(PI);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 2.0 + 0.2)
                        * Quaternion::rotation_y(-1.0)
                        * Quaternion::rotation_z(0.0);

                    next.control.orientation =
                        Quaternion::rotation_x(-1.4) * Quaternion::rotation_y(-2.8);

                    next.head.orientation = Quaternion::rotation_x(move1 * 0.2);
                    next.jaw.position = Vec3::new(0.0, s_a.jaw.0, s_a.jaw.1);
                    next.jaw.orientation = Quaternion::rotation_x(move2 * -0.3);

                    let twist = move1 * 0.8 + move3 * -0.8;
                    next.upper_torso.position = Vec3::new(
                        0.0,
                        s_a.upper_torso.0,
                        s_a.upper_torso.1 + move1 * 1.0 + move2 * -1.0,
                    );
                    next.upper_torso.orientation =
                        Quaternion::rotation_x(move1 * 0.8 + move2 * -1.1)
                            * Quaternion::rotation_z(twist * -0.2 + move1 * -0.1 + move2 * 0.3);

                    next.lower_torso.orientation =
                        Quaternion::rotation_x(move1 * -0.8 + move2 * 1.1)
                            * Quaternion::rotation_z(-twist + move1 * 0.4);

                    next.shoulder_l.position = Vec3::new(
                        -s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothorir * 1.0,
                    );
                    next.shoulder_l.orientation = Quaternion::rotation_x(-0.4);

                    next.shoulder_r.position = Vec3::new(
                        s_a.shoulder.0 + move2 * -2.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2,
                    );
                    next.shoulder_r.orientation = Quaternion::rotation_y(move1 * -PI / 2.0)
                        * Quaternion::rotation_x(move2 * 2.0)
                        * Quaternion::rotation_z(move1 * -PI / 2.0);

                    next.hand_r.position = Vec3::new(
                        -s_a.grip.1 + move1 * -2.0 + move2 * 8.0,
                        0.0 + move1 * 6.0,
                        s_a.grip.0 + move1 * 18.0 + move2 * -19.0,
                    );
                    next.hand_r.orientation = Quaternion::rotation_x(move1 * -3.0 + move2 * 3.0)
                        * Quaternion::rotation_y(move1 * 0.5 + move2 * -1.5)
                        * Quaternion::rotation_z(move1 * -1.5);

                    if speed == 0.0 {
                        next.leg_l.orientation = Quaternion::rotation_x(move1 * 0.8 + move2 * -0.8);

                        next.foot_l.position = Vec3::new(
                            -s_a.foot.0,
                            s_a.foot.1,
                            s_a.foot.2 + move1 * 4.0 + move2 * -4.0,
                        );
                        next.foot_l.orientation =
                            Quaternion::rotation_x(move1 * -0.6 + move2 * 0.6);
                    }
                },
                Some("common.abilities.vampire.strigoi.projectiles") => {
                    let (move1, move2, move3) = match stage_section {
                        Some(StageSection::Buildup) => (anim_time.powf(0.25), 0.0, 0.0),
                        Some(StageSection::Action) => (1.0, anim_time, 0.0),
                        Some(StageSection::Recover) => (1.0, 1.0, anim_time.powi(4)),
                        _ => (0.0, 0.0, 0.0),
                    };
                    next.control_l.position = Vec3::new(1.0, 2.0, 8.0);
                    next.control_r.position = Vec3::new(1.0, 1.0, -2.0);

                    next.control.position =
                        Vec3::new(-7.0, 0.0 + s_a.grip.0 / 1.0, -s_a.grip.0 / 0.8);

                    next.control_l.orientation =
                        Quaternion::rotation_x(PI / 2.0) * Quaternion::rotation_z(PI);
                    next.control_r.orientation = Quaternion::rotation_x(PI / 2.0 + 0.2)
                        * Quaternion::rotation_y(-1.0)
                        * Quaternion::rotation_z(0.0);

                    next.control.orientation =
                        Quaternion::rotation_x(-1.4) * Quaternion::rotation_y(-2.8);

                    next.head.orientation = Quaternion::rotation_x(move1 * 0.2);
                    next.head.position = Vec3::new(
                        0.0 + move1 * 32.0 - move2 * 32.0,
                        s_a.head.0 - move1 * 2.0 + move2 * 2.0,
                        s_a.head.1 - move1 * 8.0 + move2 * 8.0,
                    );

                    next.jaw.position = Vec3::new(0.0, s_a.jaw.0, s_a.jaw.1);
                    next.jaw.orientation = Quaternion::rotation_x(move2 * -0.3);

                    let twist = move1 * 0.8 + move3 * -0.8;
                    next.upper_torso.position = Vec3::new(
                        0.0,
                        s_a.upper_torso.0,
                        s_a.upper_torso.1 + move1 * 1.0 + move2 * -1.0,
                    );
                    next.upper_torso.orientation =
                        Quaternion::rotation_x(move1 * 0.8 + move2 * -1.1)
                            * Quaternion::rotation_z(twist * -0.2 + move1 * -0.1 + move2 * 0.3);

                    next.lower_torso.orientation =
                        Quaternion::rotation_x(move1 * -0.8 + move2 * 1.1)
                            * Quaternion::rotation_z(-twist + move1 * 0.4);

                    next.shoulder_l.position = Vec3::new(
                        -s_a.shoulder.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2 - foothorir * 1.0,
                    );
                    next.shoulder_l.orientation = Quaternion::rotation_x(-0.4);

                    next.shoulder_r.position = Vec3::new(
                        s_a.shoulder.0 + move2 * -2.0,
                        s_a.shoulder.1,
                        s_a.shoulder.2,
                    );
                    next.shoulder_r.orientation = Quaternion::rotation_y(move1 * -PI / 2.0)
                        * Quaternion::rotation_x(move2 * 2.0)
                        * Quaternion::rotation_z(move1 * -PI / 2.0);

                    next.hand_r.position = Vec3::new(
                        -s_a.grip.1 + move1 * -2.0 + move2 * 8.0,
                        0.0 + move1 * 6.0,
                        s_a.grip.0 + move1 * 18.0 + move2 * -19.0,
                    );
                    next.hand_r.orientation = Quaternion::rotation_x(move1 * -3.0 + move2 * 3.0)
                        * Quaternion::rotation_y(move1 * 0.5 + move2 * -1.5)
                        * Quaternion::rotation_z(move1 * -1.5);

                    if speed == 0.0 {
                        next.leg_l.orientation = Quaternion::rotation_x(move1 * 0.8 + move2 * -0.8);

                        next.foot_l.position = Vec3::new(
                            -s_a.foot.0,
                            s_a.foot.1,
                            s_a.foot.2 + move1 * 4.0 + move2 * -4.0,
                        );
                        next.foot_l.orientation =
                            Quaternion::rotation_x(move1 * -0.6 + move2 * 0.6);
                    }
                },
                _ => {},
            },
            _ => {},
        }

        next
    }
}
