use bevy::prelude::*;
use thiserror::Error;

use crate::asset_tracking::LoadResource;

pub(crate) fn plugin(app: &mut App) {
    app.register_type::<SuburbanBuildings>();
    app.load_resource::<SuburbanBuildings>();
    app.register_type::<BuildingHandle>();
}

#[derive(Component, Reflect, Clone, Debug, PartialEq, Eq)]
#[reflect(Component)]
pub enum BuildingType {
    Residential(ResidentialType),
    Driveway(DrivewayType),
    Fence(FenceType),
    Path(PathType),
    Planter,
    Tree(TreeType),
}

pub fn get_handle_from_building_type(
    ty: &BuildingType,
    buildings: &SuburbanBuildings,
) -> Option<Handle<Gltf>> {
    buildings.get(ty).map(|handle| handle.handle().clone_weak())
}

#[derive(Resource, Clone, Debug)]
pub struct CurrentBuilding {
    ty: BuildingType,
    model: Handle<Gltf>,
    scene: Handle<Scene>,
    mat_id: AssetId<StandardMaterial>,
}

#[derive(Error, Debug)]
pub enum CurrentBuildingError {
    #[error("Building type {ty:?} does not have a valid handle")]
    Handle { ty: BuildingType },
    #[error("Invalid asset. Could not retrieve {ty:?} asset with id {id:?}")]
    GltfAsset { ty: BuildingType, id: AssetId<Gltf> },
    #[error("Gltf asset {model:?} has no scenes!")]
    Scenes { model: Handle<Gltf> },
    #[error("Gltf asset {model:?} has no materials!")]
    Materials { model: Handle<Gltf> },
}

impl CurrentBuilding {
    pub fn new(
        ty: &BuildingType,
        buildings: &SuburbanBuildings,
        gltf: &Assets<Gltf>,
        materials: &mut Assets<StandardMaterial>,
    ) -> Result<Self, CurrentBuildingError> {
        let Some(handle) = get_handle_from_building_type(ty, buildings) else {
            return Err(CurrentBuildingError::Handle { ty: ty.clone() });
        };

        let id = handle.id();
        let Some(model) = gltf.get(id) else {
            return Err(CurrentBuildingError::GltfAsset { ty: ty.clone(), id });
        };

        let Some(scene) = model.scenes.first() else {
            return Err(CurrentBuildingError::Scenes {
                model: handle.clone_weak(),
            });
        };

        let Some(mat) = model.materials.first() else {
            return Err(CurrentBuildingError::Materials {
                model: handle.clone_weak(),
            });
        };

        let mat_id = mat.id();
        if let Some(mat_asset) = materials.get_mut(mat_id) {
            mat_asset.diffuse_transmission = 0.4;
        } else {
            warn!(
                "Invalid material asset. Could not retrieve {ty:?}'s material with id {mat_id:?}"
            );
        };

        Ok(Self {
            ty: ty.clone(),             // clone if it's `Clone`
            model: handle.clone_weak(), // store the handle
            scene: scene.clone_weak(),  // store the handle
            mat_id,
        })
    }

    pub fn ty(&self) -> &BuildingType {
        &self.ty
    }

    pub fn model(&self) -> &Handle<Gltf> {
        &self.model
    }

    pub fn scene(&self) -> &Handle<Scene> {
        &self.scene
    }

    pub fn mat_id(&self) -> AssetId<StandardMaterial> {
        self.mat_id
    }
}

#[derive(Reflect, Clone, Debug, PartialEq, Eq)]
pub enum ResidentialType {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
}

#[derive(Reflect, Clone, Debug, PartialEq, Eq)]
pub enum DrivewayType {
    Long,
    Short,
}

#[derive(Reflect, Clone, Debug, PartialEq, Eq)]
pub enum FenceType {
    OneByTwo,
    OneByThree,
    OneByFour,
    TwoByTwo,
    TwoByThree,
    ThreeByTwo,
    ThreeByThree,
    Low,
    Regular,
}

#[derive(Reflect, Clone, Debug, PartialEq, Eq)]
pub enum PathType {
    Long,
    Short,
    StonesLong,
    StonesMessy,
    StonesShort,
}

#[derive(Reflect, Clone, Debug, PartialEq, Eq)]
pub enum TreeType {
    Large,
    Small,
}

#[derive(Asset, Reflect, Clone, Debug, PartialEq, Eq)]
pub struct BuildingHandle {
    name: String,
    handle: Handle<Gltf>,
}

impl BuildingHandle {
    pub fn new(name: &str, handle: Handle<Gltf>) -> Self {
        Self {
            name: name.to_string(),
            handle,
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn handle(&self) -> &Handle<Gltf> {
        &self.handle
    }
}

#[derive(Resource, Asset, Clone, Reflect)]
#[reflect(Resource)]
pub struct SuburbanBuildings {
    pub buildings: Vec<(BuildingType, BuildingHandle)>,
}

impl SuburbanBuildings {
    pub fn get(&self, building_type: &BuildingType) -> Option<&BuildingHandle> {
        self.buildings.iter().find_map(|(btype, handle)| {
            if btype == building_type {
                Some(handle)
            } else {
                None
            }
        })
    }
}

impl FromWorld for SuburbanBuildings {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        let buildings = vec![
            (
                BuildingType::Residential(ResidentialType::A),
                BuildingHandle::new(
                    "building-type-a",
                    asset_server.load("models/buildings/building-type-a.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::B),
                BuildingHandle::new(
                    "building-type-b",
                    asset_server.load("models/buildings/building-type-b.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::C),
                BuildingHandle::new(
                    "building-type-c",
                    asset_server.load("models/buildings/building-type-c.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::D),
                BuildingHandle::new(
                    "building-type-d",
                    asset_server.load("models/buildings/building-type-d.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::E),
                BuildingHandle::new(
                    "building-type-e",
                    asset_server.load("models/buildings/building-type-e.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::F),
                BuildingHandle::new(
                    "building-type-f",
                    asset_server.load("models/buildings/building-type-f.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::G),
                BuildingHandle::new(
                    "building-type-g",
                    asset_server.load("models/buildings/building-type-g.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::H),
                BuildingHandle::new(
                    "building-type-h",
                    asset_server.load("models/buildings/building-type-h.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::I),
                BuildingHandle::new(
                    "building-type-i",
                    asset_server.load("models/buildings/building-type-i.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::J),
                BuildingHandle::new(
                    "building-type-j",
                    asset_server.load("models/buildings/building-type-j.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::K),
                BuildingHandle::new(
                    "building-type-k",
                    asset_server.load("models/buildings/building-type-k.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::L),
                BuildingHandle::new(
                    "building-type-l",
                    asset_server.load("models/buildings/building-type-l.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::M),
                BuildingHandle::new(
                    "building-type-m",
                    asset_server.load("models/buildings/building-type-m.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::N),
                BuildingHandle::new(
                    "building-type-n",
                    asset_server.load("models/buildings/building-type-n.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::O),
                BuildingHandle::new(
                    "building-type-o",
                    asset_server.load("models/buildings/building-type-o.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::P),
                BuildingHandle::new(
                    "building-type-p",
                    asset_server.load("models/buildings/building-type-p.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::Q),
                BuildingHandle::new(
                    "building-type-q",
                    asset_server.load("models/buildings/building-type-q.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::R),
                BuildingHandle::new(
                    "building-type-r",
                    asset_server.load("models/buildings/building-type-r.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::S),
                BuildingHandle::new(
                    "building-type-s",
                    asset_server.load("models/buildings/building-type-s.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::T),
                BuildingHandle::new(
                    "building-type-t",
                    asset_server.load("models/buildings/building-type-t.glb"),
                ),
            ),
            (
                BuildingType::Residential(ResidentialType::U),
                BuildingHandle::new(
                    "building-type-u",
                    asset_server.load("models/buildings/building-type-u.glb"),
                ),
            ),
            (
                BuildingType::Driveway(DrivewayType::Long),
                BuildingHandle::new(
                    "driveway-long",
                    asset_server.load("models/buildings/driveway-long.glb"),
                ),
            ),
            (
                BuildingType::Driveway(DrivewayType::Short),
                BuildingHandle::new(
                    "driveway-short",
                    asset_server.load("models/buildings/driveway-short.glb"),
                ),
            ),
            (
                BuildingType::Fence(FenceType::OneByTwo),
                BuildingHandle::new(
                    "fence-1x2",
                    asset_server.load("models/buildings/fence-1x2.glb"),
                ),
            ),
            (
                BuildingType::Fence(FenceType::OneByThree),
                BuildingHandle::new(
                    "fence-1x3",
                    asset_server.load("models/buildings/fence-1x3.glb"),
                ),
            ),
            (
                BuildingType::Fence(FenceType::OneByFour),
                BuildingHandle::new(
                    "fence-1x4",
                    asset_server.load("models/buildings/fence-1x4.glb"),
                ),
            ),
            (
                BuildingType::Fence(FenceType::TwoByTwo),
                BuildingHandle::new(
                    "fence-2x2",
                    asset_server.load("models/buildings/fence-2x2.glb"),
                ),
            ),
            (
                BuildingType::Fence(FenceType::TwoByThree),
                BuildingHandle::new(
                    "fence-2x3",
                    asset_server.load("models/buildings/fence-2x3.glb"),
                ),
            ),
            (
                BuildingType::Fence(FenceType::ThreeByTwo),
                BuildingHandle::new(
                    "fence-3x2",
                    asset_server.load("models/buildings/fence-3x2.glb"),
                ),
            ),
            (
                BuildingType::Fence(FenceType::ThreeByThree),
                BuildingHandle::new(
                    "fence-3x3",
                    asset_server.load("models/buildings/fence-3x3.glb"),
                ),
            ),
            (
                BuildingType::Fence(FenceType::Low),
                BuildingHandle::new(
                    "fence-low",
                    asset_server.load("models/buildings/fence-low.glb"),
                ),
            ),
            (
                BuildingType::Fence(FenceType::Regular),
                BuildingHandle::new("fence", asset_server.load("models/buildings/fence.glb")),
            ),
            (
                BuildingType::Path(PathType::Long),
                BuildingHandle::new(
                    "path-long",
                    asset_server.load("models/buildings/path-long.glb"),
                ),
            ),
            (
                BuildingType::Path(PathType::Short),
                BuildingHandle::new(
                    "path-short",
                    asset_server.load("models/buildings/path-short.glb"),
                ),
            ),
            (
                BuildingType::Path(PathType::StonesLong),
                BuildingHandle::new(
                    "path-stones-long",
                    asset_server.load("models/buildings/path-stones-long.glb"),
                ),
            ),
            (
                BuildingType::Path(PathType::StonesMessy),
                BuildingHandle::new(
                    "path-stones-messy",
                    asset_server.load("models/buildings/path-stones-messy.glb"),
                ),
            ),
            (
                BuildingType::Path(PathType::StonesShort),
                BuildingHandle::new(
                    "path-stones-short",
                    asset_server.load("models/buildings/path-stones-short.glb"),
                ),
            ),
            (
                BuildingType::Planter,
                BuildingHandle::new("planter", asset_server.load("models/buildings/planter.glb")),
            ),
            (
                BuildingType::Tree(TreeType::Large),
                BuildingHandle::new(
                    "tree-large",
                    asset_server.load("models/buildings/tree-large.glb"),
                ),
            ),
            (
                BuildingType::Tree(TreeType::Small),
                BuildingHandle::new(
                    "tree-small",
                    asset_server.load("models/buildings/tree-small.glb"),
                ),
            ),
        ];
        Self { buildings }
    }
}
