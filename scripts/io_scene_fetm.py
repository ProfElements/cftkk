import bpy
import struct
from typing import Any
from itertools import pairwise
from enum import Enum
from enum import StrEnum
from bpy.types import Operator
from bpy_extras.io_utils import ImportHelper
from bpy.props import StringProperty
from mathutils import Vector
from mathutils import Quaternion

bl_info = {
    'name': 'FETM format',
}

class TokenKind(Enum):
    SBYTE = 0
    UBYTE = 1
    S16 = 2
    U16 = 3
    U32 = 4
    HEX8 = 5
    F32 = 6
    STR = 7

class Token:
    def __init__(self, kind: TokenKind, data: Any):
        self.kind = kind
        self.data = data 
    def __repr__(self):
        return f'Token({self.kind},{self.data})'

class NodeKind(StrEnum):
    PROP = 'prop' 
    SIMULATION_OBJECT = 'simulation_object'
    DYNAMIC_LIGHT = 'dynamic_light'
    SKYBOX = 'skybox'
    SPLINE = 'spline'
    TRIGGER_BOX = 'trigger_box'
    TRIGGER_SPHERE = 'trigger_sphere'
    TRIGGER_PLANE = 'trigger_plane'
    TRIGGER_BEAM = 'trigger_beam'
    REF_POINT = 'refpoint'
    COLLISION_NODE = 'collision_node'
    SIMULATION = 'simulation'
    CONDITION = 'condition'
    CAMERA = 'camera'
    LIGHT_MATRIX = 'light_matrix'
    CELL_ARRAY = 'cellarray'
    SOUND_EMITTER = 'sound_emitter'
    AUDIO_STREAM_EMITTER = 'audiostream_emitter'
    EFFECT = 'effect'
    ROOM = 'room'
    PORTAL = 'portal'
    SPRITE_BATCHES = 'sprite_batches'
    GROUP = 'group'
    DECAL_SYSTEM = 'decalsystem'
    OVERLAY_SET = 'overlayset'
    CONTAINTER_OVERLAY = 'containeroverlay'
    TEXTURE_OVERLAY = 'textureoverlay'
    STRING_OVERLAY = 'stringoverlay'
    MESH_OVERLAY = 'meshoverlay'
    TEXTURE_BOX_OVERLAY = 'textureboxoverlay'
    NAV_MESH = 'navmesh'
    JOINT = 'joint'
    DATATABLE = 'datatable'
    SPLASH = 'splash'
    CONTROLLER = 'controller'
    NODE = 'node'
    ADVANCED_NODE = 'advancednode'
    DUMMY = 'dummy'

class Node:
    def __init__(self, kind: NodeKind, name: str):
        self.kind = kind
        self.name = name
        self.entity_class = None
        self.client = None
        self.data = []


class EntityClassKind(StrEnum):
    TRIGGER_BOX_CHECKPOINT = 'Trigger Box Checkpoint'
    TRIGGER_BOX = 'Trigger Box'
    REF_POINT_SPRITE = 'Ref Point Sprite'
    SPRITE_PICKUP = 'Sprite Pickup' 
    SPRITE_PICKUP_SNOOZEZ = 'Sprite Pickup SnoozeZ'
    SPRITE_PICKUP_SNOOZEZ_10 = 'Sprite Pickup SnoozeZ 10'
    SPRITE_PICKUP_SNOOZEZ_SPAWNABLE = 'Sprite Pickup SnoozeZ Spawnable'
    PROP_PICKUP = 'Prop Pickup'
    PROP_BASE = 'Prop Base'
    PROP_LEVEL_OBJECTIVE_PICKUP = 'Prop Level Objective Pickup'
    GAME_OBJECT = 'Game Object'
    PROP_SLEEPY_SEED = 'Prop Sleepy Seed'
    LANDSCAPE = 'Landscape'
    SCENIC = 'Scenic'
    CAMERA_BASE = 'Camera Base'
    CHASE_CAMERA = 'Chase Camera'
    TRANSITION_CAMERA = 'Transition Camera'
    OBJECT_MOVER = 'Object Mover'
    TRIGGER_BOX_OPERATOR = 'Trigger Box Operator'
    LIMITED_COLLISION_NODE = 'Limited Collision Node'
    ROOT_COLLISION_NODE = 'Root Collision Node'
    STATIC_COLLISION_NODE = 'Static Collision Node'
    DYNAMIC_COLLISION_NODE = 'Dynamic Collision Node'
    MOTION_CONTROLLED_OBJECT = 'Motion Controlled Object'
    MOTION_VECTOR_PLATFORM = 'Motion Vector Platform'
    PLATFORM = 'Platform'
    ROTATING_PLATFORM = 'Rotating Platform'
    SPLINE_PLATFORM = 'Spline Platform'
    HAZARD = 'Hazard'
    ANIM_HAZARD = 'Anim Hazard'
    DEFAULT_PARTICLE_SYSTEM = 'DefaultParticleSystem'
    SB_DEFAULT_PARTICLE_SYSTEM = 'SBDefaultParticleSystem'
    HALO_EFFECT = 'Halo Effect'
    SWIPE_EFFECT = 'Swipe Effect'
    PLUME_EFFECT = 'Plume Effect'
    GLOOP_STREAM_EFFECT = 'Gloop Stream Effect'
    COVER_AREA_ACTOR_EFFECT = 'Cover Area Actor Effect'
    ADVANCED_GROUP = 'Advanced Group'
    INTERACTIVE_BOUNCE_BUTTON = 'Interactive Bounce Button'
    INTERACTIVE_OBJECT = 'Interactive Object'
    SMART_DOOR = 'Smart Door'
    SPLINE_MANAGER = 'Spline Manager'
    NPC_EXTRA = 'NPC Extra'
    CHARACTER = 'Character'
    GLOW_EFFECT = 'Glow Effect'
    CONVERSATION = 'Conversation'
    CONVERSATION_OVERRIDE = 'Conversation Override'
    SPRINT_BLOCK = 'Sprint Block'
    FIRE_EFFECT = 'FireEffect'
    SHOCKWAVE_EFFECT = 'Shockwave Effect'
    VARIABLE_STRING_HUD_OVERLAY = 'Variable String HUD Overlay'
    VARIABLE_STRING_OVERLAY = 'Variable String Overlay'
    VARIABLE_OVERLAY_SET = 'Variable Overlay Set'
    OVERLAY_PAGING_TEXT = 'Overlay Paging Text'
    VARIABLE_INFO_OVERLAY = 'Variable Info Overlay'
    VARIABLE_DUALSTRING_OVERLAY = 'Variable DualString Overlay'
    VARIABLE_PICKUP_Z_HUD_OVERLAY = 'Variable Z Pickup HUD Overlay'
    VARIABLE_TEXTURE_OVERLAY = 'Variable Texture Overlay'
    VARIABLE_COUNTER_OVERLAY = 'Variable Counter Overlay'
    SKY_DOME = 'Sky Dome'
    DEFAULT_TRAIL_PARTICLE_SYSTEM = 'DefaultTrailParticleSystem'
    SB_DEFAULT_ACTOR_PARTICLE_SYSTEM = 'SBDefaultActorParticleSystem'
    DEFAULT_ACTOR_PARTICLE_SYSTEM = 'DefaultActorParticleSystem'
    MAIN_CHARACTER = 'Main Character'
    SUPER_NOVA_EFFECT = 'Super Nova Effect'
    EFFECT_POOL_MANAGER = 'Effect Pool Manager'
    REF_POINT_SPRITE_MANAGER = 'Ref Point Sprite Manager'
    VARIABLE_SLIDER_OVERLAY = 'Variable Slider Overlay'
    AMINATED_TEXTURE_OVERLAY = 'Animated Texture Overlay'
    GIANT_PLANKTON_COLLISION_CYLINDER = 'Giant Plankton Collision Cylinder'
    GIANT_PLANKTON_ENEMY = 'Giant Plankton Enemy'
    REF_POINT_ELECTRIC_CONDUCTOR = 'Ref Point Electric Conductor'
    ELECTRIC_EFFECT = 'Electric Effect'     
    TELEPORT_EFFECT = 'Teleport Effect'
    GIANT_PLANKTON_LAZER_EFFECT = 'Giant Plankton Lazer Effect'
    LAZER_EFFECT = 'Lazer Effect'
    SCORCH_EFFECT = 'Scorch Effect'
    TINT_VOLUME_EFFECT = 'Tint Volume Effect'
    TEXT_EFFECT = 'Text Effect'
    PROJECTILE = 'Projectile'
    DECAL_SYSTEM_ADVANCED = 'Decal System Advanced'
    JACOBS_LADDER_EFFECT = 'Jacobs Ladder Effect'
    TWISTER_EFFECT = 'Twister Effect'
    FALLING_LEAVES_EFFECT = 'Falling Leaves Effect'
    EXPLOSION_EFFECT = 'Explosion Effect'
    WIND_CONE_EFFECT = 'Wind Cone Effect'
    FLEEING_CROWD_WAYPOINT = 'Fleeing Crowd Waypoint'
    FLEEING_CROWD_MANAGER = 'Fleeing Crowd Manager'
    COVER_AREA_SPRITE_EFFECT = 'Cover Area Sprite Effect'
    OLD_FILM_EFFECT = 'Old Film Effect'
    LIGHTNING_SKY_EFFECT = 'LightningSkyEffect'
    PROP_DESTRUCTABLE_OBJECT = 'Prop Destructable Object'
    REACTIVE_OBJECT = 'Reactive Object'
    FLOATING_TEXT_OVERLAY = 'Floating Text Overlay'
    SPITE_PICKUP_GIANT_SNOOZEZ_SPAWNABLE = 'Sprite Pickup SnoozeZ Giant Spawnable'
    SPITE_PICKUP_GIANT_SNOOZEZ = 'Sprite Pickup SnoozeZ Giant'
    REF_POINT_CUTSCENE_ACTOR = 'Ref Point Cutscene Actor'
    REF_POINT_SPLINE_TRIGGER = 'Ref Point Spline Trigger'
    GIANT_PLANKTON_VEHICLE = 'Giant Plankton Vehicle'
    LIGHT_SHAFT_EFFECT = 'Light Shaft Effect'
    GIANT_PLANKTON_HELICOPTER = 'Giant Plankton Helicopter'
    NET_COPTERS_ANIM_HAZARD = 'Net Copters Anim Hazard'
    TRIP_ANIM_HAZARD = 'Trip Anim Hazard'
    CUTSCENE_CAMERA = 'Cutscene Camera'
    RAIL_CAMERA = 'Rail Camera'
    SPAWNPOINT = 'Spawn Point'
    GIANT_ROAR_PICKUP_SPAWNABLE = 'Giant Roar Pickup Spawnable'
    GIANT_ROAR_PICKUP = 'Giant Roar Pickup'
    GIANT_HEALTH_PICKUP_SPAWNABLE = 'Giant Health Pickup Spawnable'
    GIANT_HEALTH_PICKUP = 'Giant Health Pickup'
    MINI_GAME_BUSINESS_OWNERSHIP = 'Mini Game Business Ownership'
    REF_POINT_MINI_GAME = 'Ref Point Mini Game'
    REF_POINT_ROOFTOP_SHADOW = 'Ref Point RoofTop  Shadow'
    PATROLLING_ENEMY = 'Patrolling Enemy'
    PLATFORM_ENEMY = 'Platform Enemy'
    DANGEROUS_PARTICLE_SYSTEM = 'DangerousParticleSystem'
    ARMOURED_ENEMY = 'Armoured Enemy' 
    INTERACTIVE_SAVE_POINT = 'Interactive Save Point'
    INTERACTIVE_BUTTON = 'Interactive Button'
    REF_POINT_GRAPPLE_TARGET = 'Ref Point Grapple Target'
    FLASH_EFFECT = 'Flash Effect'
    BLOWABLE_OBJECT = 'Blowable Object'
    CLOUD_PARTICLE_EFFECT = 'Cloud Particle Effect'
    LIFT_PLATFORM = 'Lift Platform'
    WIND_MILL = 'Windmill'
    POWER_GENERATOR = 'Power Generator'
    TETHERED_ENEMY = 'Tethered Enemy'
    CHAIN_EFFECT = 'Chain Effect'
    TRIGGER_BOX_DEATHBOX = 'Trigger Box Deathbox'
    TRIGGER_BOX_CAMERA_HINT = 'Trigger Box Camera Hint'
    BOBBING_OBJECT = 'Bobbing Object'
    VARIABLE_GAUGE_OVERLAY = 'Variable Gauge Overlay'
    SPRITE_PICKUP_HEALTH_MAX_SPAWNABLE = 'Sprite Pickup Health Max Spawnable'
    SPRITE_PICKUP_HEALTH_MAX = 'Sprite Pickup Health Max'
    SPRITE_PICKUP_HEALTH_ONE_SPAWNABLE = 'Sprite Pickup Health 1 Spawnable'
    SPRITE_PICKUP_HEALTH_ONE = 'Sprite Pickup Health 1'
    ZIPATONE_EFFECT_LEVEL_ONE = 'Zipatone Effect Level 1'
    ZIPATONE_EFFECT_LEVEL_TWO = 'Zipatone Effect Level 2'
    ZIPATONE_EFFECT = 'Zipatone Effect'
    MELEE_ENEMY_ARMOUR = 'Melee Enemy Armour'
    DEFAULT_SPARK_EFFECT = 'DefaultSparkEffect'
    TRAIN_STOPERS_TRAIN = 'Train Stopers Train'
    TRAIN_STOPERS_MINI_GAME = 'Train Stopers Mini Game'
    TRAIN_STOPERS_CHARACTER = 'Train Stopers Character'
    INTERSECTING_PROP = 'Intersecting Prop'
    ELVIS_PATRICK_CHARACTER = 'Elvis Patrick Character'
    DREADED_PATRICK_CHARACTER = 'Dreaded Patrick Character'
    SCREEN_BLUR_EFFECT = 'Screen Blur Effect'
    CRUMBLING_PLATFORM = 'Crumbling Platform'
    CAMERA_2_5D = 'Camera 2_5D'
    BLOCK_OBJECT = 'Block Object'
    WORLD_MINI_GAME = 'World Mini Game'
    GIANT_PLANKTON_BOSS = 'Giant Plankton Boss'
    SEE_SAW_PLATFORM = 'See Saw Platform'
    DECAL_SYSTEM_VERY_ADVANCED = 'Decal System Very Advanced'
    DRIP_EFFECT = 'Drip Effect'
    PARTICLE_AREA_EFFECT = 'Particle Area Effect'
    SPRITE_PICKUP_SNOOZEZ_RED = 'Sprite Pickup SnoozeZ Red'
    SPLINE_CAMERA = 'Spline Camera'
    PROJECTILE_TRAIL_EFFECT = 'Projectile Trail Effect'
    SPEED_STREAK_AREA_EFFECT = 'Speed Streak Area Effect'
    WAFTING_FAN = 'Wafting Fan'
    FOLIAGE = 'Foliage'
    SPITTING_ENEMY = 'Spitting Enemy'
    SPLINE_PARTICLE_EFFECT = 'Spline Particle Effect'
    ENEMY_ATTACHMENT = 'Enemy Attachment'
    FLYING_SPLINE_ENEMY = 'Flying Spline Enemy'
    FLYING_ENEMY = 'Flying Enemy'
    SPLINE_TUBE_EFFECT = 'Spline Tube Effect'
    SPACE_SATELLITE_LAZER_EFFECT = 'Space Satellite Lazer Effect'
    SLOWDOWN_LIFT_PLATFORM = 'Slowdown Lift Platform'
    TARGET_POINT_EFFECT = 'Target Point Effect'
    TARGET_POINT_LIST_EFFECT = 'Target Point List Effect'
    FLYING_HOVER_ENEMY = 'Flying Hover Enemy'
    REACTIVE_OBJECT_BOSS_FLASHER = 'Reactive Object Boss Flasher'
    SPHERE_LIGHT_EFFECT = 'Sphere Light Effect'
    PROP_FUEL_PICKUP = 'Prop Fuel Pickup'
    PROP_REGEN_FUEL_PICKUP = 'Prop Regen Fuel Pickup'
    SPRITE_PICKUP_SNOOZEZ_MEDIUM_REGEN = 'Sprite Pickup SnoozeZ Medium Regen'
    SPRITE_PICKUP_SNOOZEZ_10_MEDIUM_REGEN = 'Sprite Pickup SnoozeZ 10 Medium Regen'
    SPRITE_PICKUP_SNOOZEZ_MEDIUM = 'Sprite Pickup SnoozeZ Medium'
    SPRITE_PICKUP_SNOOZEZ_10_MEDIUM = 'Sprite Pickup SnoozeZ 10 Medium'
    DRIVING_SPEEDUP_PAD = 'Driving Speedup Pad'
    DRIVING_CAMERA = 'Driving Camera'
    VARIABLE_COLON_ALIGNED_TIMER = 'Variable Colon Aligned Timer'
    DRIVING_GENERAL_MESSAGE_OVERLAY_SET = 'Driving General Message Overlay Set'
    DRIVING_COUNTDOWN_OVERLAY = 'Driving Countdown Overlay'
    HOTROD_SMOKE_EFFECT = 'HotRod Smoke Effect'
    SKIDMARK_EFFECT = 'Skidmark Effect'
    DRIVING_OPPONENT = 'Driving Opponent'
    SPRITE_PICKUP_SNOOZEZ_LARGE = 'Sprite Pickup SnoozeZ Large'
    SELECTABLE_CHARACTER = 'Selectable Character'
    CHARACTER_SELECTION_CONTROLLER = 'Character Selection Controller'
    FULL_FRONTEND_OVERLAY = 'FullFrontEndOverlay'
    GAME_CREDITS_OVERLAY = 'Game Credits Overlay'
    ROOFTOP_SHRINKRAY = 'RoofTop ShrinkRay'
    ROOFTOP_SATELLITE = 'RoofTop Satellite'
    ROOFTOP_PLANKTON = 'RoofTop Plankton'
    ROOFTOP_PROJECTILE = 'Rooftop Projectile'
    ROOFTOP_MANAGER = 'RoofTop Manager'
    ROOFTOP_BREAKABLE_WINDOW = 'RoofTop Breakable Window'
    REF_POINT_SPRITE_ASTEROID = 'Ref Point Sprite Asteroid'
    MINI_GAME_OVERLAY_SET = 'Mini Game Overlay Set'
    OVERLAY_FRONTEND_BUBBLE_LABEL = 'Overlay FrontEnd Bubble Label'
    PROP_LOCKED_FRONTEND_MENU_BUBBLE = 'Prop Locked Frontend Menu Bubble'
    PROP_FRONTEND_MENU_BUBBLE = 'Prop FrontEnd Menu Bubble'
    GROUP_FRONTEND_MENU = 'Group FrontEnd Menu'
    BUBBLE_PARTICLE_SYSTEM = 'BubbleParticleSystem'

    def size(self) -> int:
        if self == EntityClassKind.TRIGGER_BOX_CHECKPOINT:
            # trigger box data + entity_class data
            return EntityClassKind.TRIGGER_BOX.size() + 3 
        elif self == EntityClassKind.TRIGGER_BOX:
            # name + entity_class_header + entity_class_data 
            return 1 + 4 + 2
        elif self == EntityClassKind.REF_POINT_SPRITE:
            # name + entity_class_header + entity_class_data
            return 1 + 4 + 10
        elif self == EntityClassKind.SPRITE_PICKUP:
            # ref point sprite data + 
            return EntityClassKind.REF_POINT_SPRITE.size() + 11
        elif self == EntityClassKind.SPRITE_PICKUP_SNOOZEZ:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.SPRITE_PICKUP_SNOOZEZ_10:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.PROP_SLEEPY_SEED:
            return EntityClassKind.PROP_PICKUP.size() + 1
        elif self == EntityClassKind.PROP_PICKUP:
            return EntityClassKind.GAME_OBJECT.size() + 11
        elif self == EntityClassKind.GAME_OBJECT:
            return EntityClassKind.PROP_BASE.size() + 7
        elif self == EntityClassKind.PROP_BASE:
            # name + header + data
            return 1 + 4 + 39 
        elif self == EntityClassKind.LANDSCAPE:
            return EntityClassKind.PROP_BASE.size() + 2
        elif self == EntityClassKind.SCENIC:
            return EntityClassKind.PROP_BASE.size()
        elif self == EntityClassKind.CAMERA_BASE:
            #TODO: this can produce actions list, we will eventually have to do this properly
            return 1 + 4 + 31
        elif self == EntityClassKind.OBJECT_MOVER:
            #TODO: this can produce actions list, we will eventually have to do this properly
            return EntityClassKind.GAME_OBJECT.size() + 46
        elif self == EntityClassKind.TRIGGER_BOX_OPERATOR:
            return EntityClassKind.TRIGGER_BOX.size() + 2
        elif self == EntityClassKind.LIMITED_COLLISION_NODE:
            return 1 + 4
        elif self == EntityClassKind.MOTION_CONTROLLED_OBJECT:
            #TODO: this can produce actions list, we will eventually have to do this properly
            return EntityClassKind.GAME_OBJECT.size() + 27
        elif self == EntityClassKind.ROOT_COLLISION_NODE:
            return 1 + 4
        elif self == EntityClassKind.STATIC_COLLISION_NODE:
            return 1 + 4
        elif self == EntityClassKind.DYNAMIC_COLLISION_NODE:
            return 1 + 4
        elif self == EntityClassKind.MOTION_VECTOR_PLATFORM:
            return EntityClassKind.PLATFORM.size() + 13
        elif self == EntityClassKind.PLATFORM:
            return EntityClassKind.GAME_OBJECT.size() + 19
        elif self == EntityClassKind.SPLINE_PLATFORM:
            return EntityClassKind.PLATFORM.size() + 19  
        elif self == EntityClassKind.HAZARD:
            return EntityClassKind.GAME_OBJECT.size() + 52
        elif self == EntityClassKind.DEFAULT_PARTICLE_SYSTEM:
            return 1 + 4 + 51
        elif self == EntityClassKind.SB_DEFAULT_PARTICLE_SYSTEM:
            return EntityClassKind.DEFAULT_PARTICLE_SYSTEM.size() + 39 
        elif self == EntityClassKind.ANIM_HAZARD:
            return EntityClassKind.HAZARD.size() + 8 
        elif self == EntityClassKind.HALO_EFFECT:
            return 1 + 4 + 23
        elif self == EntityClassKind.SWIPE_EFFECT:
            return 1 + 4 + 25
        elif self == EntityClassKind.PLUME_EFFECT:
            return 1 + 4 + 23
        elif self == EntityClassKind.GLOOP_STREAM_EFFECT:
            return 1 + 4 + 23
        elif self == EntityClassKind.ADVANCED_GROUP:
            return 1 + 4
        elif self == EntityClassKind.INTERACTIVE_BOUNCE_BUTTON:
            #TODO: this can produce actions list, we will eventually have to do this properly
            return EntityClassKind.INTERACTIVE_OBJECT.size() + 19
        elif self == EntityClassKind.INTERACTIVE_OBJECT:
            #TODO: this can produce actions list, we will eventually have to do this properly
            return EntityClassKind.GAME_OBJECT.size() + 12
        elif self == EntityClassKind.SMART_DOOR:
            return EntityClassKind.GAME_OBJECT.size() + 13
        elif self == EntityClassKind.SPLINE_MANAGER:
            return 1 + 4 + 3
        elif self == EntityClassKind.NPC_EXTRA:
            return EntityClassKind.CHARACTER.size() + 11 
        elif self == EntityClassKind.CHARACTER:
            return EntityClassKind.GAME_OBJECT.size() + 30
        elif self == EntityClassKind.GLOW_EFFECT:
            return 1 + 4 + 45
        elif self == EntityClassKind.ROTATING_PLATFORM:
            return EntityClassKind.PLATFORM.size() + 3
        elif self == EntityClassKind.CONVERSATION:
            return 1 + 4 + 10
        elif self == EntityClassKind.CONVERSATION_OVERRIDE:
            return 1 + 4
        elif self == EntityClassKind.PROP_LEVEL_OBJECTIVE_PICKUP:
            return EntityClassKind.PROP_PICKUP.size()
        elif self == EntityClassKind.SPRINT_BLOCK:
            return EntityClassKind.GAME_OBJECT.size() + 5
        elif self == EntityClassKind.FIRE_EFFECT:
            return 1 + 4 + 17
        elif self == EntityClassKind.VARIABLE_STRING_HUD_OVERLAY:
            return EntityClassKind.VARIABLE_STRING_OVERLAY.size() + 2 
        elif self == EntityClassKind.VARIABLE_STRING_OVERLAY:
            return EntityClassKind.VARIABLE_OVERLAY_SET.size() + 8
        elif self == EntityClassKind.VARIABLE_OVERLAY_SET:
            return 1 + 4 + 28
        elif self == EntityClassKind.OVERLAY_PAGING_TEXT:
            return 1 + 4 + 3
        elif self == EntityClassKind.VARIABLE_INFO_OVERLAY:
            return EntityClassKind.VARIABLE_OVERLAY_SET.size() + 2
        elif self == EntityClassKind.VARIABLE_DUALSTRING_OVERLAY:
            return EntityClassKind.VARIABLE_STRING_OVERLAY.size() + 2
        elif self == EntityClassKind.VARIABLE_PICKUP_Z_HUD_OVERLAY:
            return EntityClassKind.VARIABLE_STRING_OVERLAY.size() + 2
        elif self == EntityClassKind.VARIABLE_TEXTURE_OVERLAY:
            return EntityClassKind.VARIABLE_OVERLAY_SET.size() + 7
        elif self == EntityClassKind.VARIABLE_COUNTER_OVERLAY:
            return EntityClassKind.VARIABLE_TEXTURE_OVERLAY.size() + 9
        elif self == EntityClassKind.SKY_DOME:
            return EntityClassKind.PROP_BASE.size() + 69
        elif self == EntityClassKind.DEFAULT_TRAIL_PARTICLE_SYSTEM:
            return EntityClassKind.DEFAULT_PARTICLE_SYSTEM.size() + 4
        elif self == EntityClassKind.COVER_AREA_ACTOR_EFFECT:
            return EntityClassKind.SB_DEFAULT_ACTOR_PARTICLE_SYSTEM.size() + 11
        elif self == EntityClassKind.SB_DEFAULT_ACTOR_PARTICLE_SYSTEM:
            return EntityClassKind.DEFAULT_ACTOR_PARTICLE_SYSTEM.size() + 40
        elif self == EntityClassKind.DEFAULT_ACTOR_PARTICLE_SYSTEM:
            return EntityClassKind.DEFAULT_PARTICLE_SYSTEM.size() + 2
        elif self == EntityClassKind.MAIN_CHARACTER:
            return EntityClassKind.CHARACTER.size() + 107
        elif self == EntityClassKind.SHOCKWAVE_EFFECT:
            return 1 + 4 + 14
        elif self == EntityClassKind.SUPER_NOVA_EFFECT:
            return 1 + 4 + 44
        elif self == EntityClassKind.EFFECT_POOL_MANAGER:
            return 1 + 4
        elif self == EntityClassKind.REF_POINT_SPRITE_MANAGER:
            return 1 + 4
        elif self == EntityClassKind.CHASE_CAMERA:
            return EntityClassKind.CAMERA_BASE.size() + 9
        elif self == EntityClassKind.TRANSITION_CAMERA:
            return EntityClassKind.CAMERA_BASE.size()
        elif self == EntityClassKind.SPRITE_PICKUP_SNOOZEZ_SPAWNABLE:
            return EntityClassKind.SPRITE_PICKUP_SNOOZEZ.size()
        elif self == EntityClassKind.VARIABLE_SLIDER_OVERLAY:
            return EntityClassKind.VARIABLE_OVERLAY_SET.size() + 5
        elif self == EntityClassKind.AMINATED_TEXTURE_OVERLAY:
            return 1 + 4
        elif self == EntityClassKind.GIANT_PLANKTON_COLLISION_CYLINDER:
            return EntityClassKind.GIANT_PLANKTON_ENEMY.size() + 4
        elif self == EntityClassKind.GIANT_PLANKTON_ENEMY:
            return EntityClassKind.CHARACTER.size()
        elif self == EntityClassKind.REF_POINT_ELECTRIC_CONDUCTOR:
            return 1 + 4 + 1
        elif self == EntityClassKind.ELECTRIC_EFFECT:
            return 1 + 4 + 58
        elif self == EntityClassKind.TELEPORT_EFFECT:
            return 1 + 4 + 37
        elif self == EntityClassKind.GIANT_PLANKTON_LAZER_EFFECT:
            return EntityClassKind.LAZER_EFFECT.size()
        elif self == EntityClassKind.LAZER_EFFECT:
            return 1 + 4 + 33
        elif self == EntityClassKind.SCORCH_EFFECT:
            return 1 + 4 + 11
        elif self == EntityClassKind.PROJECTILE:
            return EntityClassKind.GAME_OBJECT.size() + 28
        elif self == EntityClassKind.DECAL_SYSTEM_ADVANCED:
            return 1 + 4 + 15
        elif self == EntityClassKind.JACOBS_LADDER_EFFECT:
            return EntityClassKind.ELECTRIC_EFFECT.size() + 8
        elif self == EntityClassKind.TWISTER_EFFECT:
            return 1 + 4 + 10 
        elif self == EntityClassKind.FALLING_LEAVES_EFFECT:
            return 1 + 4 + 30
        elif self == EntityClassKind.EXPLOSION_EFFECT:
            return 1 + 4 + 44
        elif self == EntityClassKind.FLEEING_CROWD_WAYPOINT:
            return 1 + 4 + 10
        elif self == EntityClassKind.FLEEING_CROWD_MANAGER:
            return 1 + 4 + 18
        elif self == EntityClassKind.COVER_AREA_SPRITE_EFFECT:
            return EntityClassKind.SB_DEFAULT_PARTICLE_SYSTEM.size() + 11
        elif self == EntityClassKind.OLD_FILM_EFFECT:
            return 1 + 4 + 20
        elif self == EntityClassKind.LIGHTNING_SKY_EFFECT:
            return 1 + 4 + 27
        elif self == EntityClassKind.PROP_DESTRUCTABLE_OBJECT:
            return EntityClassKind.REACTIVE_OBJECT.size() + 21
        elif self == EntityClassKind.REACTIVE_OBJECT:
            return EntityClassKind.GAME_OBJECT.size() + 61
        elif self == EntityClassKind.FLOATING_TEXT_OVERLAY:
            return 1 + 4 + 4
        elif self == EntityClassKind.SPITE_PICKUP_GIANT_SNOOZEZ_SPAWNABLE:
            return EntityClassKind.SPRITE_PICKUP_SNOOZEZ.size()
        elif self == EntityClassKind.SPITE_PICKUP_GIANT_SNOOZEZ:
            return EntityClassKind.SPRITE_PICKUP_SNOOZEZ.size()
        elif self == EntityClassKind.REF_POINT_CUTSCENE_ACTOR:
            return 1 + 4 + 7
        elif self == EntityClassKind.GIANT_PLANKTON_VEHICLE:
            return EntityClassKind.GIANT_PLANKTON_ENEMY.size() + 4
        elif self == EntityClassKind.TINT_VOLUME_EFFECT: 
            return 1 + 4 + 10 
        elif self == EntityClassKind.LIGHT_SHAFT_EFFECT:
            return 1 + 4 + 12
        elif self == EntityClassKind.GIANT_PLANKTON_HELICOPTER:
            return EntityClassKind.GIANT_PLANKTON_ENEMY.size() + 4
        elif self == EntityClassKind.NET_COPTERS_ANIM_HAZARD:
            return EntityClassKind.ANIM_HAZARD.size() + 3
        elif self == EntityClassKind.REF_POINT_SPLINE_TRIGGER:
            return 1 + 4
        elif self == EntityClassKind.CUTSCENE_CAMERA:
            return EntityClassKind.CAMERA_BASE.size() + 12
        elif self == EntityClassKind.RAIL_CAMERA:
            return EntityClassKind.CAMERA_BASE.size() + 5
        elif self == EntityClassKind.SPAWNPOINT:
            return 1 + 4
        elif self == EntityClassKind.TRIP_ANIM_HAZARD:
            return EntityClassKind.ANIM_HAZARD.size() + 1
        elif self == EntityClassKind.GIANT_ROAR_PICKUP_SPAWNABLE:
            return EntityClassKind.GIANT_ROAR_PICKUP.size()
        elif self == EntityClassKind.GIANT_ROAR_PICKUP:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.MINI_GAME_BUSINESS_OWNERSHIP:
            return EntityClassKind.REF_POINT_MINI_GAME.size() + 8
        elif self == EntityClassKind.REF_POINT_MINI_GAME:
            return 1 + 4 + 8
        elif self == EntityClassKind.GIANT_HEALTH_PICKUP_SPAWNABLE:
            return EntityClassKind.GIANT_HEALTH_PICKUP.size()
        elif self == EntityClassKind.GIANT_HEALTH_PICKUP:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.REF_POINT_ROOFTOP_SHADOW:
            return 1 + 4 + 5
        elif self == EntityClassKind.PATROLLING_ENEMY:
            return EntityClassKind.PLATFORM_ENEMY.size() + 16
        elif self == EntityClassKind.PLATFORM_ENEMY:
            return EntityClassKind.CHARACTER.size() + 6
        elif self == EntityClassKind.TEXT_EFFECT:
            return 1 + 4 + 62
        elif self == EntityClassKind.DANGEROUS_PARTICLE_SYSTEM:
            return EntityClassKind.SB_DEFAULT_PARTICLE_SYSTEM.size() + 10
        elif self == EntityClassKind.WIND_CONE_EFFECT:
            return 1 + 4 + 12
        elif self == EntityClassKind.ARMOURED_ENEMY:
            return EntityClassKind.PATROLLING_ENEMY.size() + 3
        elif self == EntityClassKind.INTERACTIVE_SAVE_POINT:
            return EntityClassKind.INTERACTIVE_BUTTON.size()
        elif self == EntityClassKind.INTERACTIVE_BUTTON:
            return EntityClassKind.INTERACTIVE_OBJECT.size() + 23
        elif self == EntityClassKind.REF_POINT_GRAPPLE_TARGET:
            return 1 + 4
        elif self == EntityClassKind.FLASH_EFFECT:
            return 1 + 4 + 113
        elif self == EntityClassKind.BLOWABLE_OBJECT:
            return EntityClassKind.INTERACTIVE_OBJECT.size() + 43
        elif self == EntityClassKind.CLOUD_PARTICLE_EFFECT:
            return EntityClassKind.DANGEROUS_PARTICLE_SYSTEM.size() + 18
        elif self == EntityClassKind.LIFT_PLATFORM:
            return EntityClassKind.PLATFORM.size() + 5
        elif self == EntityClassKind.WIND_MILL:
            return EntityClassKind.POWER_GENERATOR.size() + 12
        elif self == EntityClassKind.POWER_GENERATOR:
            return EntityClassKind.OBJECT_MOVER.size() + 11
        elif self == EntityClassKind.TETHERED_ENEMY:
            return EntityClassKind.PLATFORM_ENEMY.size() + 5
        elif self == EntityClassKind.CHAIN_EFFECT:
            return 1 + 4 + 16
        elif self == EntityClassKind.TRIGGER_BOX_DEATHBOX:
            return EntityClassKind.TRIGGER_BOX.size() + 1
        elif self == EntityClassKind.TRIGGER_BOX_CAMERA_HINT:
            return EntityClassKind.TRIGGER_BOX.size() + 6
        elif self == EntityClassKind.BOBBING_OBJECT:
            return EntityClassKind.GAME_OBJECT.size() + 17
        elif self == EntityClassKind.VARIABLE_GAUGE_OVERLAY:
            return EntityClassKind.VARIABLE_OVERLAY_SET.size() + 3
        elif self == EntityClassKind.SPRITE_PICKUP_HEALTH_MAX_SPAWNABLE:
            return EntityClassKind.SPRITE_PICKUP_HEALTH_MAX.size()
        elif self == EntityClassKind.SPRITE_PICKUP_HEALTH_MAX:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.SPRITE_PICKUP_HEALTH_ONE_SPAWNABLE:
            return EntityClassKind.SPRITE_PICKUP_HEALTH_ONE.size()
        elif self == EntityClassKind.SPRITE_PICKUP_HEALTH_ONE:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.ZIPATONE_EFFECT_LEVEL_ONE:
            return EntityClassKind.ZIPATONE_EFFECT.size();
        elif self == EntityClassKind.ZIPATONE_EFFECT_LEVEL_TWO:
            return EntityClassKind.ZIPATONE_EFFECT.size();
        elif self == EntityClassKind.ZIPATONE_EFFECT:
            return 1 + 4 + 3
        elif self == EntityClassKind.MELEE_ENEMY_ARMOUR:
            return EntityClassKind.GAME_OBJECT.size() + 1
        elif self == EntityClassKind.DEFAULT_SPARK_EFFECT:
            return 1 + 4 + 29
        elif self == EntityClassKind.TRAIN_STOPERS_TRAIN:
            return EntityClassKind.GAME_OBJECT.size() + 21
        elif self == EntityClassKind.TRAIN_STOPERS_MINI_GAME:
            return EntityClassKind.REF_POINT_MINI_GAME.size() + 18
        elif self == EntityClassKind.INTERSECTING_PROP:
            return EntityClassKind.GAME_OBJECT.size()
        elif self == EntityClassKind.ELVIS_PATRICK_CHARACTER:
            return EntityClassKind.TRAIN_STOPERS_CHARACTER.size() + 4
        elif self == EntityClassKind.DREADED_PATRICK_CHARACTER:
            return EntityClassKind.TRAIN_STOPERS_CHARACTER.size() + 6
        elif self == EntityClassKind.TRAIN_STOPERS_CHARACTER:
            return EntityClassKind.CHARACTER.size() + 2
        elif self == EntityClassKind.SCREEN_BLUR_EFFECT:
            return 1 + 4 + 16
        elif self == EntityClassKind.CRUMBLING_PLATFORM:
            return EntityClassKind.PLATFORM.size() + 5
        elif self == EntityClassKind.CAMERA_2_5D:
            return EntityClassKind.CAMERA_BASE.size() + 4
        elif self == EntityClassKind.BLOCK_OBJECT:
            return EntityClassKind.GAME_OBJECT.size() + 2
        elif self == EntityClassKind.WORLD_MINI_GAME:
            return EntityClassKind.REF_POINT_MINI_GAME.size() + 3
        elif self == EntityClassKind.GIANT_PLANKTON_BOSS:
            return EntityClassKind.CHARACTER.size() + 7
        elif self == EntityClassKind.SEE_SAW_PLATFORM:
            return EntityClassKind.PLATFORM.size() + 4
        elif self == EntityClassKind.DECAL_SYSTEM_VERY_ADVANCED:
            return EntityClassKind.DECAL_SYSTEM_ADVANCED.size() + 14
        elif self == EntityClassKind.DRIP_EFFECT:
            return 1 + 4 + 13
        elif self == EntityClassKind.PARTICLE_AREA_EFFECT:
            return 1 + 4 + 40
        elif self == EntityClassKind.SPRITE_PICKUP_SNOOZEZ_RED:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.SPLINE_CAMERA:
            return EntityClassKind.CAMERA_BASE.size() + 1
        elif self == EntityClassKind.PROJECTILE_TRAIL_EFFECT:
            return 1 + 4 + 6
        elif self == EntityClassKind.SPEED_STREAK_AREA_EFFECT:
            return 1 + 4 + 27
        elif self == EntityClassKind.WAFTING_FAN:
            return EntityClassKind.GAME_OBJECT.size()
        elif self == EntityClassKind.FOLIAGE:
            return EntityClassKind.PROP_BASE.size() + 1
        elif self == EntityClassKind.SPITTING_ENEMY:
            return EntityClassKind.PLATFORM_ENEMY.size() + 4
        elif self == EntityClassKind.SPLINE_PARTICLE_EFFECT:
            return EntityClassKind.DEFAULT_PARTICLE_SYSTEM.size() + 8
        elif self == EntityClassKind.ENEMY_ATTACHMENT:
            return EntityClassKind.GAME_OBJECT.size()
        elif self == EntityClassKind.FLYING_SPLINE_ENEMY:
            return EntityClassKind.FLYING_ENEMY.size() + 3
        elif self == EntityClassKind.FLYING_ENEMY:
            return EntityClassKind.CHARACTER.size() + 4
        elif self == EntityClassKind.SPLINE_TUBE_EFFECT:
            return 1 + 4 + 25
        elif self == EntityClassKind.SPACE_SATELLITE_LAZER_EFFECT:
            return EntityClassKind.LAZER_EFFECT.size()
        elif self == EntityClassKind.SLOWDOWN_LIFT_PLATFORM:
            return EntityClassKind.LIFT_PLATFORM.size() + 2
        elif self == EntityClassKind.TARGET_POINT_EFFECT:
            return 1 + 4 + 13
        elif self == EntityClassKind.FLYING_HOVER_ENEMY:
            return EntityClassKind.FLYING_ENEMY.size() + 10
        elif self == EntityClassKind.TARGET_POINT_LIST_EFFECT:
            return 1 + 4 + 5
        elif self == EntityClassKind.REACTIVE_OBJECT_BOSS_FLASHER:
            return EntityClassKind.REACTIVE_OBJECT.size() + 1
        elif self == EntityClassKind.SPHERE_LIGHT_EFFECT:
            return EntityClassKind.TINT_VOLUME_EFFECT.size() + 1
        elif self == EntityClassKind.PROP_FUEL_PICKUP:
            return EntityClassKind.PROP_PICKUP.size()
        elif self == EntityClassKind.PROP_REGEN_FUEL_PICKUP:
            return EntityClassKind.PROP_FUEL_PICKUP.size()
        elif self == EntityClassKind.SPRITE_PICKUP_SNOOZEZ_MEDIUM_REGEN:
            return EntityClassKind.SPRITE_PICKUP_SNOOZEZ_MEDIUM.size()
        elif self == EntityClassKind.SPRITE_PICKUP_SNOOZEZ_MEDIUM:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.DRIVING_SPEEDUP_PAD:
            return EntityClassKind.GAME_OBJECT.size() + 3
        elif self == EntityClassKind.SPRITE_PICKUP_SNOOZEZ_10_MEDIUM_REGEN:
            return EntityClassKind.SPRITE_PICKUP_SNOOZEZ_10_MEDIUM.size()
        elif self == EntityClassKind.SPRITE_PICKUP_SNOOZEZ_10_MEDIUM:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.DRIVING_CAMERA:
            return EntityClassKind.CAMERA_BASE.size() + 5
        elif self == EntityClassKind.VARIABLE_COLON_ALIGNED_TIMER:
            return EntityClassKind.VARIABLE_OVERLAY_SET.size() + 3
        elif self == EntityClassKind.DRIVING_GENERAL_MESSAGE_OVERLAY_SET:
            return 1 + 4 + 2
        elif self == EntityClassKind.DRIVING_COUNTDOWN_OVERLAY:
            return 1 + 4 + 2
        elif self == EntityClassKind.HOTROD_SMOKE_EFFECT:
            return 1 + 4 + 36
        elif self == EntityClassKind.SKIDMARK_EFFECT:
            return 1 + 4 + 3
        elif self == EntityClassKind.DRIVING_OPPONENT:
            return EntityClassKind.CHARACTER.size() + 4
        elif self == EntityClassKind.SPRITE_PICKUP_SNOOZEZ_LARGE:
            return EntityClassKind.SPRITE_PICKUP.size()
        elif self == EntityClassKind.SELECTABLE_CHARACTER:
            return EntityClassKind.GAME_OBJECT.size() + 3
        elif self == EntityClassKind.CHARACTER_SELECTION_CONTROLLER:
            return 1 + 4 + 9
        elif self == EntityClassKind.FULL_FRONTEND_OVERLAY:
            return 1 + 4 + 4
        elif self == EntityClassKind.GAME_CREDITS_OVERLAY:
            return 1 + 4 + 5
        elif self == EntityClassKind.ROOFTOP_SHRINKRAY:
            return EntityClassKind.ROOFTOP_SATELLITE.size() + 2
        elif self == EntityClassKind.ROOFTOP_SATELLITE:
            return EntityClassKind.GAME_OBJECT.size() + 7
        elif self == EntityClassKind.ROOFTOP_PLANKTON:
            return EntityClassKind.GIANT_PLANKTON_BOSS.size() + 3
        elif self == EntityClassKind.ROOFTOP_PROJECTILE:
            return EntityClassKind.PROJECTILE.size() + 1
        elif self == EntityClassKind.ROOFTOP_MANAGER:
            return 1 + 4 + 5
        elif self == EntityClassKind.ROOFTOP_BREAKABLE_WINDOW:
            return EntityClassKind.GAME_OBJECT.size()
        elif self == EntityClassKind.REF_POINT_SPRITE_ASTEROID:
            return EntityClassKind.REF_POINT_SPRITE.size()
        elif self == EntityClassKind.MINI_GAME_OVERLAY_SET:
            return 1 + 4 + 12
        elif self == EntityClassKind.OVERLAY_FRONTEND_BUBBLE_LABEL:
            return 1 + 4 + 4
        elif self == EntityClassKind.PROP_LOCKED_FRONTEND_MENU_BUBBLE:
            return EntityClassKind.PROP_FRONTEND_MENU_BUBBLE.size() + 7
        elif self == EntityClassKind.PROP_FRONTEND_MENU_BUBBLE:
            return 1 + 4 + 27
        elif self == EntityClassKind.GROUP_FRONTEND_MENU:
            return 1  + 4 + 24
        elif self == EntityClassKind.BUBBLE_PARTICLE_SYSTEM:
            return EntityClassKind.DEFAULT_PARTICLE_SYSTEM.size()

class EntityClass:
    def __init__(self, kind: EntityClassKind, data: list[Token]):
        self.kind = kind
        self.data = data

class ClientKind(StrEnum):
    C_PROP_ROOFTOP_PLANKTON = 'CPropRoofTopPlankton'
    C_PROP_ROOFTOP_SHRINKRAY = 'CPropRoofTopShrinkRay'
    C_PROP_ROOFTOP_PROJECTILE = 'CPropRoofTopProjectile'
    C_ROOFTOP_BREAKABLE_WINDOW = 'CPropRoofTopBreakableWindow'
    C_ROOFTOP_SATELLITE = 'CPropRoofTopSatellite'
    C_REF_POINT_SPRITE = 'CRefPointSprite'
    C_TRIGGER_BOX = 'CTriggerBox'
    C_TRIGGER_BOX_CHECKPOINT = 'CTriggerBoxCheckpoint'
    C_TRIGGER_BOX_CAMERA_HINT = 'CTriggerBoxCameraHint'
    C_TRIGGER_BOX_DEATH_BOX = 'CTriggerBoxDeathbox'
    C_REF_POINT_SPRITE_PICKUP = 'CRefPointSpritePickup'
    C_PROP_BASE = 'CPropBase'
    C_PROP_SLEEPY_SEED = 'CPropSleepySeed'
    C_PROP_SCENIC = 'CPropScenic'
    C_PROP_LANDSCAPE = 'CPropLandscape'
    C_CAMERA_BASE_NODE = 'CCameraBaseNode'
    C_CHASE_CAMERA_NODE = 'CChaseCameraNode'
    C_TRANSITION_CAMERA_NODE = 'CTransitionCameraNode'
    C_CUTSCENE_CAMERA_NODE = 'CCutsceneCameraNode'
    C_RAIL_CAMERA_Node = 'CRailCameraNode'
    C_PROP_OBJECT_MOVER = 'CPropObjectMover'
    C_TRIGGER_BOX_OPERATOR = 'CTriggerBoxOperator'
    C_PROP_MOTION_CONTROLLED_OBJECT = 'CPropMotionControlledObject'
    C_PROP_MOTION_VECTOR_PLATFORM = 'CPropMotionVectorPlatform'
    C_PROP_SPLINE_PLATFORM = 'CPropSplinePlatform'
    C_PROP_HAZARD = 'CPropHazard'
    C_PROP_ANIM_HAZARD = 'CPropAnimHazard'
    C_PROP_INTERACTIVE_BOUNCE_BUTTON = 'CPropInteractiveBounceButton'
    C_PROP_SMART_DOOR = 'CPropSmartDoor'
    C_PROP_CHARACTER = 'CPropCharacter'
    C_PROP_NPC_EXTRA = 'CPropNPCExtra'
    C_PROP_ROTATING_PLATFORM = 'CPropRotatingPlatform'
    C_PROP_LEVEL_OBJECTIVE_PICKUP = 'CPropLevelObjectivePickup'
    C_PROP_SKYDOME = 'CPropSkyDome'
    C_PROP_MAIN_CHARACTER = 'CPropMainCharacter'
    C_PROP_POWER_GENERATOR = 'CPropPowerGenerator'
    C_PROP_DRIVING_SPEEDUP_PAD = 'CPropDrivingSpeedupPad'
    PARTICLE_SYTEM = 'CFWorldNodeParticleSystem'
    TRAIL_PARTICLE_SYTEM = 'CFWorldNodeTrailParticleSystem'
    SBPARTICLE_SYTEM = 'CSBDefaultParticleSystem'
    C_FX_HALO = 'CFxHalo'
    C_FX_PLUME = 'CFxPlume'
    C_FX_GLOOP_STREAM = 'CFxGloopStream'
    C_FX_SHOCKWAVE = 'CFxShockwave'
    C_FX_ELECTRIC = 'CFxElectric'
    C_FX_LAZER = 'CFxLazer'
    C_FX_SCORCH = 'CFxScorch'
    C_FX_JACOBS_LADDER = 'CFxJacobsLadder'
    C_FX_TWISTER = 'CFxTwister'
    C_FX_EXPLOSION = 'CFxExplosion'
    C_FX_OLD_FILM = 'CFxOldFilm'
    C_FX_TINT_VOLUME = 'CFxTintVolume'
    C_FX_LIGHT_SHAFT = 'CFxLightShaft'
    C_FX_DANGEROUS_PARTICLE = 'CFxDangerousParticle'
    C_FX_WIND_CONE = 'CFxWindCone'
    C_FX_ZIPATONE = 'CFxZipatone'
    C_FX_DRIP = 'CFxDrip'
    C_FX_PARTICLE_AREA = 'CFxParticleArea'
    C_FX_PROJECTILE_TRAIL = 'CFxProjectileTrail'
    C_FX_SPEED_STREAK_AREA = 'CFxSpeedStreakArea'
    C_FX_SPLINE_PARTICLE_SYSTEM = 'CFxSplineParticleSystem'
    C_FX_SPLINE_TUBE = 'CFxSplineTube'
    C_FX_TARGET_POINT = 'CFxTargetPoint'
    C_FX_TARGET_POINT_LIST = 'CFxTargetPointList'
    C_FX_SPHERE_TINT_VOLUME = 'CFxSphereTintVolume'
    C_SWIPE_EFFECT = 'CSwipeEffect'
    C_GLOW_EFFECT = 'CGlowEffect'
    C_FIRE_EFFECT = 'CFireEffect'
    C_TELEPORT_EFFECT = 'CTeleportEffect'
    C_SUPER_NOVA_EFFECT = 'CSuperNovaEffect'
    C_COVER_AREA_ACTOR_EFFECT = 'CCoverAreaActorEffect'
    C_COVER_AREA_SPRITE_EFFECT = 'CCoverAreaSpriteEffect'
    C_LIGHTNING_SKY_EFFECT = 'CLightningSkyEffect'
    C_TEXT_EFFECT = 'CTextEffect'
    C_FLASH_EFFECT = 'CFlashEffect'
    C_CLOUD_PARTICLE_EFFECT = 'CCloudParticleEffect'
    C_CHAIN_EFFECT = 'CChainEffect'
    C_ADVANCED_GROUP = 'CAdvancedGroup'
    C_SPLINE_MANAGER = 'CSplineManager'
    C_CONVERSATION = 'CConversation'
    C_PROP_SPRINT_BLOCK = 'CPropSprintBlock'
    C_OVERLAY_VARIABLE_STRING = 'COverlayVariableString'
    C_OVERLAY_VARIABLE_DUAL_STRING = 'COverlayVariableDualString'
    C_OVERLAY_VARIABLE_HUD_STRING = 'COverlayVariableHUDString'
    C_OVERLAY_SUBTITLE_SCROLLING_TEXT = 'COverlaySubtitleScrolling'
    C_OVERLAY_VARIABLE_INFO = 'COverlayVariableInfo'
    C_OVERLAY_VARIABLE_Z_PICKUP = 'COverlayVariableZPickup'
    C_OVERLAY_VARIABLE_COUNTER = 'COverlayVariableCounter'
    C_OVERLAY_VARIABLE_TEXTURE = 'COverlayVariableTexture'
    C_EFFECT_POOL_MANAGER = 'CEffectPoolManager'
    C_SPRITE_MANAGER = 'CSpriteManager'
    C_OVERLAY_VARIABLE_SLIDER = 'COverlayVariableSlider'
    C_ANIMATED_TEXTURE_OVERLAY = 'CAnimatedTextureOverlay'
    C_PROP_GIANT_PLANKTON_COLLISION_CYLINDER = 'CPropGiantPlanktonCollisionCylinder'
    C_PROP_GIANT_PLANKTON_VEHICLE_ENEMY = 'CPropGiantPlanktonVehicleEnemy'
    C_PROP_GIANT_PLANKTON_HELICOPTER = 'CPropGiantPlanktonHelicopter'
    C_PROP_NET_COPTER_ANIM_HAZARD = 'CPropNetCopterAnimHazard'   
    C_PROP_TRIP_ANIM_HAZARD = 'CPropTripAnimHazard'   
    C_REF_POINT_ELECTRIC_CONDUCTOR = 'CRefPointElectricConductor'
    C_REF_POINT_GRAPPLE_TARGET = 'CRefPointGrappleTarget'
    C_PROP_PROJECTILE = 'CPropProjectile'
    C_DECAL_SYSTEM_ADVANCED = 'CDecalSystemAdvanced'
    C_FALLING_LEAVES = 'CFallingLeaves'
    C_FLEEING_CROWD_WAYPOINT = 'CFleeingCrowdWaypoint'
    C_FLEEING_CROWD_MANAGER = 'CFleeingCrowdManager'
    C_PROP_DESTRUCTION_OBJECT = 'CPropDestructionObject'
    C_PROP_INTERSECTING_OBJECT = 'CPropIntersectingObject'
    C_OVERLAY_FLOATING_TEXT = 'COverlayFloatingText'
    C_OVERLAY_VARIABLE_OVERLAY = 'COverlayVariableGauge'
    C_PROP_PICKUP = 'CPropPickup'
    C_PROP_ROOFTOP_SHADOW = 'CPropRoofTopShadow'
    C_REF_POINT_CUTSCENE_ACTOR = 'CRefPointCutsceneActor'
    C_REF_POINT_SPLINE_TRIGGER = 'CRefPointSplineTrigger'
    C_REF_POINT_SPAWN = 'CRefPointSpawn'
    C_MINI_GAME_BUSINESS_OWNERSHIP = 'CMiniGame_BusinessOwnership'
    C_MINI_GAME_WORLD_MINI_GAME = 'CMiniGame_WorldMiniGame'
    C_PROP_PATROLLING_ENEMY = 'CPropPatrollingEnemy'
    C_PROP_ARMOURED_ENEMY = 'CPropArmouredEnemy'
    C_PROP_INTERACTIVE_SAVE_POINT = 'CPropInteractiveSavePoint'
    C_PROP_BLOWABLE_OBJECT = 'CPropBlowableObject'
    C_PROP_LIFT_PLATFORM = 'CPropLiftPlatform'
    C_PROP_WINDMILL = 'CPropWindmill'
    C_PROP_TETHERED_ENEMY = 'CPropTetheredEnemy'
    C_PROP_BOBBING_OBJECT = 'CPropBobbingObject'
    C_PROP_MELEE_ENEMY_ARMOUR = 'CPropMeleeEnemyArmour'
    C_PROP_REACTIVE_OBJECT = 'CPropReactiveObject'
    C_PROP_INTERACTIVE_BUTTON = 'CPropInteractiveButton'
    C_PROP_INTERACTIVE_OBJECT = 'CPropInteractiveObject'
    C_PROP_GAME_OBJECT = 'CPropGameObject'
    C_PROP_PLATFORM = 'CPropPlatform'
    CF_WORLD_NODE_SPARK_EFFECT = 'CFWorldNodeSparkEffect'
    C_TRAIN_STOPERS_TRAIN = 'CTrainStopersTrain'
    C_TRAIN_STOPERS_CONTROLLER = 'CTrainStopersController'
    C_TRAIN_STOPERS_CHARACTER_DREADED_PATRICK = 'CTrainStopersCharacter_DreadedPatrick'
    C_TRAIN_STOPERS_CHARACTER_ELVIS_PATRICK = 'CTrainStopersCharacter_ElvisPatrick'
    C_SCREEN_BLUR = 'CScreenBlur'
    C_PROP_CRUMBLING_PLATFORM = 'CPropCrumblingPlatform'
    C_CAMERA_2_5D = 'CCamera2_5DNode'
    C_PROP_BLOCK = 'CPropBlock'
    C_PROP_GIANT_PLANKTON_BOSS = 'CPropGiantPlanktonBoss'
    C_PROP_SEE_SAW_PLATFORM = 'CPropSeeSawPlatform'
    C_DECAL_SYSTEM_VERY_ADVANCED = 'CDecalSystemVeryAdvanced'
    C_SPLINE_CAMERA = 'CSplineCamera'
    C_PROP_WAFTING_FAN = 'CPropWaftingFan'
    C_PROP_FOLIAGE = 'CPropFoliage'
    C_PROP_SPITTING_ENEMY = 'CPropSpittingEnemy'
    C_PROP_ENEMY_ATTACHMENT = 'CPropEnemyAttachment'
    C_PROP_FLYING_SPLINE_ENEMY = 'CPropFlyingSplineEnemy'
    C_SB_DEFAULT_ACTOR_PARTICLE_SYSTEM = 'CSBDefaultActorParticleSystem'
    ACTOR_PARTICLE_SYSTEM = 'CFWorldNodeActorParticleSystem'
    C_PROP_SLOWDOWN_LIFT_PLATFORM = 'CPropSlowdownLiftPlatform'
    C_PROP_FLYING_HOVER_ENEMY = 'CPropFlyingHoverEnemy'
    EMPTY = ''
    C_DRIVING_CAMERA_NODE = 'CDrivingCameraNode'    
    C_OVERLAY_COLON_ALIGNED_TIMER = 'COverlayColonAlignedTimer'
    C_OVERLAY_DRIVING_GENERAL_MESSAGE = 'COverlayDrivingGeneralMessage'
    C_OVERLAY_DRIVING_START_COUNTDOWN = 'COverlayDrivingStartCountdown'
    C_PROP_DRIVING_OPPONENT = 'CPropDrivingOpponent'
    C_PROP_SELECTABLE_CHARACTER = 'CPropSelectableCharacter'
    C_CHARACTER_SELECTION_CONTROLLER = 'CCharacterSelectionController'
    C_FULL_FRONTEND_OVERLAY = 'CFullFrontEndOverlay'
    C_GAME_CREDITS_OVERLAY = 'CGameCreditsOverlay'
    C_MINI_GAME_OVERLAY_SET = 'CMiniGameOverlaySet'
    C_OVERLAY_FRONTEND_BUBBLE_LABEL = 'COverlayFrontEndBubbleLabel'
    C_PROP_LOCKED_FRONTEND_MENU_BUBBLE = 'CPropLockedFrontEndMenuBubble'
    C_PROP_FRONTEND_MENU_BUBBLE = 'CPropFrontEndMenuBubble'
    C_GROUP_FRONTEND_MENU = 'CGroupFrontEndMenu'
    C_FULL_FRONTEND_BUBBLE_PARTICLE = 'CFullFrontEndBubbleParticle'

class Client:
    def __init__(self, kind: ClientKind, data: list[Token]):
        self.kind = kind
        self.data = data

PRINT_ENTITY_CLASS = False
def read_fetm(context, filepath):
    with open(filepath, 'rb') as f:
        data = memoryview(f.read())
        tokens = []
        idx = 0
        size = len(data);
        while idx < len(data):
            #print(idx)
            kind = data[idx]
            if kind == 0:
               signed_8bit = int.from_bytes(data[idx+1:idx+1].tobytes(), "big", signed=True)
               tokens.append(Token(TokenKind(kind), signed_8bit))
               #print("data: " + str(signed_8bit))
               idx = idx + 2
            elif kind == 1: 
               tokens.append(Token(kind, data[idx+1]))
               #print("data: " + str(data[idx+1]))
               idx = idx + 2
            elif kind == 2:
                signed_16bit = int.from_bytes(data[idx+1:idx+3].tobytes(), "big", signed=True)
                tokens.append(Token(TokenKind(kind), signed_16bit)) 
                #print("data: " + str(signed_16bit))
                idx = idx + 3
            elif kind == 3:
                unsigned_16bit = int.from_bytes(data[idx+1:idx+3].tobytes(), "big", signed=False)
                tokens.append(Token(TokenKind(kind), unsigned_16bit)) 
                #print("data: " + str(unsigned_16bit))
                idx = idx + 3
            elif kind == 4:
                unsigned_32bit = int.from_bytes(data[idx+1:idx+5].tobytes(), "big", signed=False)
                tokens.append(Token(TokenKind(kind), unsigned_32bit)) 
                #print("data: " + str(unsigned_32bit))
                idx = idx + 5
            elif kind == 5:
                unsigned_32bit = int.from_bytes(data[idx+1:idx+5].tobytes(), "big", signed=False)
                tokens.append(Token(TokenKind(kind), unsigned_32bit)) 
                #print("data: " + str(unsigned_32bit))
                idx = idx +5
            elif kind == 6:
                float_32bit = struct.unpack('>f', data[idx+1:idx+5].tobytes())[0]
                tokens.append(Token(TokenKind(kind), float_32bit)) 
                #print("data: " + str(float_32bit))
                idx = idx + 5
            elif kind == 7:
                end = data[idx + 1];
                end_idx = idx + 1;
                while end != 0:
                     end = data[end_idx]
                     end_idx += 1
                is_end = data[idx+1:end_idx-1].tobytes().decode()
                tokens.append(Token(TokenKind(kind), is_end))
                #print("data: " + is_end)
                    
                if end_idx + 1 >= len(data):
                    break;
                if end_idx < size and data[end_idx] == 0:
                    end_idx = end_idx + 1
                idx = end_idx
            else:
                print("unknown kind: " + str(kind) + " " + str(data[idx:].tobytes()))
                
                break 
        idx = 0
        nodes = []
        while idx < len(tokens):
            token = tokens[idx]
            if idx+1 >= len(tokens):
                break;
            token_2 = tokens[idx+1]
            if type(token.data) is str and token.data in [e.value for e in NodeKind] and type(token_2.data) is str:     
                kind = NodeKind(token.data)
                name = str(token_2.data)
                node = Node(kind, name)
                if name == '<noentclass>':
                    print(f"{name}")
                else:
                    nodes.append((node, idx))    
            idx = idx + 1  

        for ((node, idx), (node2, idx2)) in pairwise(nodes):
            node.data = tokens[idx:idx2] 
        node_to_obj = [] 
        for (node, idx) in nodes:
            if not node.data:
                node.data = tokens[idx:]
            if len(node.data) < 3:
                node.data = tokens[idx:]
             
            obj = bpy.data.objects.new(node.name, None)
            if node.data[2].data == '<noentclass>' and node.data[3].data == '':
                transform = transform_from_tokens(node.data[4:])
                if transform is not None:
                    #print(node.kind + " | " + node.name + "\n" + str(node.data))
                    obj.location = transform.pos
                    obj.scale = transform.scale
                    obj.rotation_quaternion = transform.rotation
            else: 
                if node.data[2].data in [e.value for e in EntityClassKind]:
                    kind = EntityClassKind(node.data[2].data)
                    if kind is not None: 
                        ec_len = kind.size()
                        ec_len = min(ec_len, node.data[2+4].data + 5)
                        entity_class_data = node.data[2:2+ec_len]
                        #print("enty_class data " + str(entity_class_data))
                        node.entity_class = EntityClass(kind, entity_class_data)
                        #print("found entity_class: " + str(node.entity_class.kind))
                        idx = 2 + len(node.entity_class.data)
                        #print("data: " + str(node.data[idx:]))
                        #print("Maybe client class type: " + str(node.data[idx-1].data);
                        if PRINT_ENTITY_CLASS:
                            print("Maybe client class type: " + str(node.data[idx].data))
                        while idx < len(node.data):
                            if node.data[idx].data in [e.value for e in ClientKind]:
                                break
                            else:
                                idx = idx + 1
                                #print("Maybe client class type: " + str(node.data[idx].data))
                        if PRINT_ENTITY_CLASS:
                            print("Maybe client class type: " + str(node.data[idx].data))
                        if idx < len(node.data) and node.data[idx].data in [e.value for e in ClientKind]:
                            client = ClientKind(node.data[idx].data)
                            if client is not None:
                                transform = transform_from_tokens(node.data[idx+1:])
                                bounds = bounds_from_tokens(node.data[idx+1+12:])
                                if transform is not None:
                                    obj.location = transform.pos
                                    #obj.scale = transform.scale
                                    obj.rotation_quaternion = transform.rotation
                                if bounds is not None:
                                    max_b = bounds.max
                                    min_b = bounds.min
                                    if max > obj.location:
                                        max_b = max_b - obj.location
                                        min_b = min_b - obj.location
                                    obj.empty_display_type = 'CUBE'                   
                                    obj.empty_display_size = (max_b - min_b).length / 2
                        else:
                            print("client idx mismatch: " + str(node.data[idx:])  + " " + str(node.data[2:]))
                else:
                    print("Unrecognized entity_class: " + str(node.data[2].data))
                    print(f"{node.name} with kind {node.kind} has {node.data}")
            bpy.context.collection.objects.link(obj)
            node_to_obj.append((node.name, obj.name))
        model_names = []
        for (node, idx) in nodes:
            if node.kind == NodeKind.PROP or node.kind == NodeKind.SIMULATION_OBJECT:
                    idx = 0 
                    for token in node.data:
                        if token.kind == TokenKind.STR:
                            idx = idx + 1
                            if idx == 6 and token.data != '<nomesh>' and token.data != '':
                                model_names.append((node, token.data));
                                idx = 0
        errors = []
        for (node, name) in model_names:
            print(node.name + " wants " + name) 
            FILEPATH =  "/home/profelements/assets/"
            file = FILEPATH + name + ".actr.obj"
            try:
                bpy.ops.wm.obj_import(filepath=file, up_axis='Z', forward_axis='Y')
                name = node.name
                while idx < len(node_to_obj):
                    (node_name, obj_name) = node_to_obj[idx]
                    if node_name == name:
                        name = obj_name
                        break;
                    idx = idx + 1
                try:
                    obj = bpy.context.collection.objects[name]
                except KeyError:
                    obj = bpy.context.collection.objects[name + ".001"]
                bpy.context.view_layer.objects.active.parent = obj
            except RuntimeError as run: 
                errors.append(file + " is missing")
        for error in errors:
            print(error)
        return {'FINISHED'}

class Transform:
    def __init__(self, pos: Vector, scale: Vector, rot: Quaternion):
        self.pos = pos
        self.scale = scale
        self.rotation = rot

def transform_from_tokens(tokens: list[Token]): 
    kind = tokens[0].data
    
    if len(tokens) < 11:
        pos = Vector()
        scale = Vector((1.0, 1.0, 1.0))
        rotation = Quaternion()
        return Transform(pos, scale, rotation)

    if kind == 1:
        pos = Vector((tokens[1].data, tokens[2].data, tokens[3].data))
        scale = Vector((tokens[4].data, tokens[5].data, tokens[6].data))
        rotation = Quaternion((tokens[10].data, tokens[7].data, tokens[8].data, tokens[9].data))
        return Transform(pos, scale, rotation)

class Bounds:
    def __init__(self, min: Vector, max: Vector):
        self.min = min
        self.max = max

def bounds_from_tokens(tokens: list[Token]):
    min = Vector((tokens[0].data, tokens[1].data, tokens[2].data))
    max = Vector((tokens[3].data, tokens[4].data, tokens[5].data))
    #print(str(min) + " to " + str(max))
    return Bounds(min, max)
class FETMImporter(Operator, ImportHelper):
    bl_idname = "import_scene.fetm"
    bl_label = "Import Blitz Games .fet/.fetm"
    
    filename_ext = ".fetm"
    
    filter_glob = StringProperty(
        default="*.fetm",
        options={'HIDDEN'},
        maxlen=255,
    )
    
    def execute(self, context):
        return read_fetm(context, self.filepath)

def menu_func_import(self, context):
    self.layout.operator(FETMImporter.bl_idname, text=FETMImporter.bl_label);

def register():
    bpy.utils.register_class(FETMImporter)
    bpy.types.TOPBAR_MT_file_import.append(menu_func_import)

def unregister():
    bpy.utils.unregister_class(FETMImporter)
    bpy.types.TOPBAR_MT_file_import.remove(menu_func_import)
    
if __name__ == "__main__":
    register()
